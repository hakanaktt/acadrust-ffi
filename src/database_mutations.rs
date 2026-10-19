//! Native database record construction and structural mutations.

use std::ffi::{c_char, c_void};

use acadrust::objects as obj;
use acadrust::tables::TableEntry;
use acadrust::{CadDocument, EntityType, Handle};

use crate::views::{view, NativeView};
use crate::*;

macro_rules! object_catalog {
    ($($name:literal => $make:expr),+ $(,)?) => {
        const OBJECT_NAMES: &[&str] = &[$($name),+];
        pub(crate) fn make_object(name: &str) -> FfiResult<obj::ObjectType> {
            match name.to_ascii_uppercase().as_str() {
                $($name => Ok($make),)+
                "IMAGEDEF" => make_object("IMAGEDEFINITION"),
                "IMAGEDEF_REACTOR" => make_object("IMAGEDEFINITIONREACTOR"),
                "MLEADERSTYLE" => make_object("MULTILEADERSTYLE"),
                "DICTIONARYVAR" => make_object("DICTIONARYVARIABLE"),
                "ACDBDICTIONARYWDFLT" => make_object("DICTIONARYWITHDEFAULT"),
                "ACDBPLACEHOLDER" => make_object("PLACEHOLDER"),
                _ => Err(invalid("unknown native object kind; enumerate ar_object_kind_name")),
            }
        }
    };
}

fn dynamic_object(data: obj::DynamicBlockData, name: &str, cpp: &str) -> obj::ObjectType {
    let name = data.entity_dxf_name().unwrap_or(name).to_owned();
    let cpp = data
        .entity_cpp_name()
        .map(str::to_owned)
        .unwrap_or_else(|| {
            if cpp != "AcDbObject" {
                return cpp.to_owned();
            }
            match &data {
                obj::DynamicBlockData::Unknown => "AcDbObject".into(),
                obj::DynamicBlockData::Representation(_) => "AcDbBlockRepresentationData".into(),
                obj::DynamicBlockData::ProxyNode(_) => "AcDbDynamicBlockProxyNode".into(),
                obj::DynamicBlockData::GripLocationComponent(_) => "AcDbBlockGripExpr".into(),
                obj::DynamicBlockData::EvaluationGraph(_) => "AcDbEvalGraph".into(),
                obj::DynamicBlockData::SolidHistory(_) => "AcDbShHistory".into(),
                obj::DynamicBlockData::SolidHistoryNode(value) => value
                    .class_names()
                    .map(|(_, cpp)| cpp.to_owned())
                    .unwrap_or_else(|| "AcDbObject".into()),
                obj::DynamicBlockData::RotateAction(_) => "AcDbBlockRotationAction".into(),
                _ => format!("AcDbBlock{}", data.variant().expect("dynamic enum")),
            }
        });
    obj::ObjectType::DynamicBlock(obj::DynamicBlockObject {
        dxf_name: name,
        cpp_class_name: cpp,
        data,
        ..Default::default()
    })
}

fn associative_object(data: obj::AssociativeData, name: &str) -> obj::ObjectType {
    let subtype = match &data {
        obj::AssociativeData::SurfaceActionBody(v) => match v.kind {
            obj::AssocSurfaceActionKind::EdgeChamfer => "EdgeChamferActionBody".into(),
            obj::AssocSurfaceActionKind::EdgeFillet => "EdgeFilletActionBody".into(),
            _ => format!(
                "{}SurfaceActionBody",
                v.kind.variant().expect("surface action enum")
            ),
        },
        obj::AssociativeData::AnnotationActionBody(v) => match v.kind {
            obj::AssocAnnotationKind::AlignedDimension => "AlignedDimActionBody".into(),
            obj::AssocAnnotationKind::ThreePointAngularDimension => {
                "3PointAngularDimActionBody".into()
            }
            obj::AssocAnnotationKind::OrdinateDimension => "OrdinateDimActionBody".into(),
            obj::AssocAnnotationKind::RotatedDimension => "RotatedDimActionBody".into(),
            _ => format!("{}ActionBody", v.kind.variant().expect("annotation enum")),
        },
        obj::AssociativeData::ViewObjectActionParam(v) => format!(
            "{}ActionParam",
            v.kind.variant().expect("view parameter enum")
        ),
        obj::AssociativeData::ConstraintGroup(_) => "2dConstraintGroup".into(),
        obj::AssociativeData::DimensionAssociation(_) => "DimAssoc".into(),
        obj::AssociativeData::PersSubentManagerStatic(_) => "PersSubentManager".into(),
        obj::AssociativeData::ArrayParameters(_) => match name {
            "ASSOCARRAYPATHPARAMETERS" => "ArrayPathParameters",
            "ASSOCARRAYPOLARPARAMETERS" => "ArrayPolarParameters",
            "ASSOCARRAYRECTANGULARPARAMETERS" => "ArrayRectangularParameters",
            _ => "ArrayModifyParameters",
        }
        .into(),
        _ => data.variant().expect("associative enum").to_owned(),
    };
    let actual_name = match &data {
        obj::AssociativeData::SurfaceActionBody(_)
        | obj::AssociativeData::AnnotationActionBody(_)
        | obj::AssociativeData::ViewObjectActionParam(_) => {
            format!("ASSOC{}", subtype.to_ascii_uppercase())
        }
        _ => name.to_owned(),
    };
    let cpp = match &data {
        obj::AssociativeData::DimensionAssociation(_)
        | obj::AssociativeData::PersSubentManagerStatic(_) => format!("AcDb{subtype}"),
        _ => format!("AcDbAssoc{subtype}"),
    };
    obj::ObjectType::Associative(obj::AssociativeObject {
        dxf_name: actual_name,
        cpp_class_name: cpp,
        data,
        ..Default::default()
    })
}

fn history_object(operation: obj::SolidHistoryOperation) -> obj::ObjectType {
    let (name, cpp) = operation
        .class_names()
        .unwrap_or(("ACSH_HISTORY_CLASS", "AcDbShHistory"));
    dynamic_object(
        obj::DynamicBlockData::SolidHistoryNode(operation),
        name,
        cpp,
    )
}

fn context_object(kind: obj::ObjectContextKind) -> obj::ObjectType {
    obj::ObjectType::ObjectContextData(obj::ObjectContextData {
        handle: Handle::NULL,
        owner_handle: Handle::NULL,
        reactors: Vec::new(),
        xdictionary_handle: None,
        class_version: 4,
        is_default: true,
        scale: Handle::NULL,
        kind,
    })
}

fn dgn_object(data: obj::DgnLineStyleData) -> obj::ObjectType {
    obj::ObjectType::DgnLineStyle(obj::DgnLineStyleObject {
        handle: Handle::NULL,
        owner: Handle::NULL,
        reactors: Vec::new(),
        xdictionary_handle: None,
        data,
    })
}

fn dgn_component(component: obj::DgnLsComponentData) -> obj::ObjectType {
    let kind = match &component {
        obj::DgnLsComponentData::Symbol(_) => obj::DgnLsComponentType::Symbol,
        obj::DgnLsComponentData::Compound(_) => obj::DgnLsComponentType::Compound,
        obj::DgnLsComponentData::Stroke(_) => obj::DgnLsComponentType::Stroke,
        obj::DgnLsComponentData::Point(_) => obj::DgnLsComponentType::Point,
        obj::DgnLsComponentData::Internal(_) => obj::DgnLsComponentType::Internal,
    };
    dgn_object(obj::DgnLineStyleData::Component {
        kind,
        description: String::new(),
        version: 0,
        component_uid: [0; 16],
        scale: 1.0,
        property_flags: 0,
        component,
        properties: Vec::new(),
    })
}

fn surface_action(kind: obj::AssocSurfaceActionKind) -> obj::ObjectType {
    associative_object(
        obj::AssociativeData::SurfaceActionBody(obj::AssocSurfaceActionBody {
            kind,
            ..Default::default()
        }),
        "",
    )
}

fn annotation_action(kind: obj::AssocAnnotationKind) -> obj::ObjectType {
    associative_object(
        obj::AssociativeData::AnnotationActionBody(obj::AssocAnnotationActionBody {
            kind,
            ..Default::default()
        }),
        "",
    )
}

fn view_action_parameter(kind: obj::AssocViewObjectActionParamKind) -> obj::ObjectType {
    associative_object(
        obj::AssociativeData::ViewObjectActionParam(obj::AssocViewObjectActionParam {
            kind,
            ..Default::default()
        }),
        "",
    )
}

fn dimension_context(subtype: obj::DimSubtype) -> obj::ObjectType {
    context_object(obj::ObjectContextKind::Dim(obj::DimContext {
        def_pt: Default::default(),
        is_def_textloc: false,
        text_rotation: 0.0,
        block: Handle::NULL,
        b293: false,
        dimtofl: false,
        dimosxd: false,
        dimatfit: false,
        dimtix: false,
        dimtmove: false,
        override_code: 0,
        has_arrow2: false,
        flip_arrow2: false,
        flip_arrow1: false,
        subtype,
    }))
}

fn array_parameters(name: &str) -> obj::ObjectType {
    associative_object(
        obj::AssociativeData::ArrayParameters(obj::AssocArrayParameters {
            class_name: "AcDbAssocArrayItem".into(),
            ..Default::default()
        }),
        name,
    )
}

object_catalog! {
    "DICTIONARY" => obj::ObjectType::Dictionary(Default::default()),
    "LAYOUT" => obj::ObjectType::Layout(obj::Layout::new("Layout")),
    "XRECORD" => obj::ObjectType::XRecord(Default::default()),
    "GROUP" => obj::ObjectType::Group(Default::default()),
    "MLINESTYLE" => obj::ObjectType::MLineStyle(obj::MLineStyle::default()),
    "IMAGEDEFINITION" => obj::ObjectType::ImageDefinition(Default::default()),
    "UNDERLAYDEFINITION" => obj::ObjectType::UnderlayDefinition(obj::UnderlayDefinition::default()),
    "PLOTSETTINGS" => obj::ObjectType::PlotSettings(Default::default()),
    "MULTILEADERSTYLE" => obj::ObjectType::MultiLeaderStyle(Default::default()),
    "TABLESTYLE" => obj::ObjectType::TableStyle(Default::default()),
    "TABLECONTENT" => obj::ObjectType::TableContent(acadrust::entities::Table::default()),
    "SCALE" => obj::ObjectType::Scale(Default::default()),
    "OBJECTCONTEXTDATA" => obj::ObjectType::ObjectContextData(obj::ObjectContextData { handle: Default::default(), owner_handle: Default::default(), reactors: Default::default(), xdictionary_handle: Default::default(), class_version: Default::default(), is_default: Default::default(), scale: Default::default(), kind: obj::ObjectContextKind::AnnotScale }),
    "SORTENTITIESTABLE" => obj::ObjectType::SortEntitiesTable(Default::default()),
    "DICTIONARYVARIABLE" => obj::ObjectType::DictionaryVariable(Default::default()),
    "VISUALSTYLE" => obj::ObjectType::VisualStyle(Default::default()),
    "MATERIAL" => obj::ObjectType::Material(Default::default()),
    "IMAGEDEFINITIONREACTOR" => obj::ObjectType::ImageDefinitionReactor(obj::ImageDefinitionReactor::new(Handle::NULL)),
    "GEODATA" => obj::ObjectType::GeoData(Default::default()),
    "SPATIALFILTER" => obj::ObjectType::SpatialFilter(Default::default()),
    "RASTERVARIABLES" => obj::ObjectType::RasterVariables(Default::default()),
    "BOOKCOLOR" => obj::ObjectType::BookColor(Default::default()),
    "PLACEHOLDER" => obj::ObjectType::PlaceHolder(Default::default()),
    "DICTIONARYWITHDEFAULT" => obj::ObjectType::DictionaryWithDefault(Default::default()),
    "WIPEOUTVARIABLES" => obj::ObjectType::WipeoutVariables(Default::default()),
    "BLOCKVISIBILITYPARAMETER" => obj::ObjectType::BlockVisibilityParameter(Default::default()),
    "DYNAMICBLOCK" => dynamic_object(obj::DynamicBlockData::Unknown, "DYNAMICBLOCK", "AcDbObject"),
    "ASSOCIATIVE" => associative_object(obj::AssociativeData::Unknown, "ASSOCIATIVE"),
    "CLASSOBJECT" => obj::ObjectType::ClassObject(Default::default()),
    "DATAOBJECT" => obj::ObjectType::DataObject(Default::default()),
    "FIELD" => obj::ObjectType::Field(Default::default()),
    "FIELDLIST" => obj::ObjectType::FieldList(Default::default()),
    "REGISTEREDCLASS" => obj::ObjectType::RegisteredClass(Default::default()),
    "DGNLINESTYLE" => obj::ObjectType::DgnLineStyle(obj::DgnLineStyleObject { handle: Default::default(), owner: Default::default(), reactors: Default::default(), xdictionary_handle: Default::default(), data: obj::DgnLineStyleData::Definition { description: Default::default(), version: Default::default(), style_number: Default::default(), component_uid: Default::default(), is_continuous: Default::default(), unit_definition: Default::default(), unit_scale: Default::default(), units_type: Default::default(), is_element: Default::default(), is_physical: Default::default(), is_scale_independent: Default::default(), is_snappable: Default::default(), root_component: Default::default(), properties: Default::default() } }),
    "PROXYOBJECT" => obj::ObjectType::ProxyObject(Default::default()),
    "UNKNOWN" => obj::ObjectType::Unknown { type_name: "UNKNOWN".into(), handle: Handle::NULL, owner: Handle::NULL, raw_dxf_codes: None, raw_dwg_data: None, raw_dwg_handle_bits: 0, raw_dwg_version: None },
    "CLASSOBJECT_EMPTY" => obj::ObjectType::ClassObject(obj::ClassObject::new(obj::ClassObjectData::Empty)),
    "CLASSOBJECT_SPATIALINDEX" => obj::ObjectType::ClassObject(obj::ClassObject::new(obj::ClassObjectData::SpatialIndex(Default::default()))),
    "CLASSOBJECT_LAYERFILTER" => obj::ObjectType::ClassObject(obj::ClassObject::new(obj::ClassObjectData::LayerFilter(Default::default()))),
    "CLASSOBJECT_PARTIALVIEWINGINDEX" => obj::ObjectType::ClassObject(obj::ClassObject::new(obj::ClassObjectData::PartialViewingIndex(Default::default()))),
    "CLASSOBJECT_VBAPROJECT" => obj::ObjectType::ClassObject(obj::ClassObject::new(obj::ClassObjectData::VbaProject(Default::default()))),
    "CLASSOBJECT_SECTIONMANAGER" => obj::ObjectType::ClassObject(obj::ClassObject::new(obj::ClassObjectData::SectionManager(Default::default()))),
    "CLASSOBJECT_SECTIONSETTINGS" => obj::ObjectType::ClassObject(obj::ClassObject::new(obj::ClassObjectData::SectionSettings(Default::default()))),
    "CLASSOBJECT_LIGHTLIST" => obj::ObjectType::ClassObject(obj::ClassObject::new(obj::ClassObjectData::LightList(Default::default()))),
    "CLASSOBJECT_SUN" => obj::ObjectType::ClassObject(obj::ClassObject::new(obj::ClassObjectData::Sun(Default::default()))),
    "CLASSOBJECT_RENDERSETTINGS" => obj::ObjectType::ClassObject(obj::ClassObject::new(obj::ClassObjectData::RenderSettings(Default::default()))),
    "CLASSOBJECT_MENTALRAYRENDERSETTINGS" => obj::ObjectType::ClassObject(obj::ClassObject::new(obj::ClassObjectData::MentalRayRenderSettings(Default::default()))),
    "CLASSOBJECT_RAPIDRTRENDERSETTINGS" => obj::ObjectType::ClassObject(obj::ClassObject::new(obj::ClassObjectData::RapidRtRenderSettings(Default::default()))),
    "CLASSOBJECT_GRADIENTBACKGROUND" => obj::ObjectType::ClassObject(obj::ClassObject::new(obj::ClassObjectData::GradientBackground(Default::default()))),
    "CLASSOBJECT_GROUNDPLANEBACKGROUND" => obj::ObjectType::ClassObject(obj::ClassObject::new(obj::ClassObjectData::GroundPlaneBackground(Default::default()))),
    "CLASSOBJECT_IBLBACKGROUND" => obj::ObjectType::ClassObject(obj::ClassObject::new(obj::ClassObjectData::IblBackground(Default::default()))),
    "CLASSOBJECT_IMAGEBACKGROUND" => obj::ObjectType::ClassObject(obj::ClassObject::new(obj::ClassObjectData::ImageBackground(Default::default()))),
    "CLASSOBJECT_SKYLIGHTBACKGROUND" => obj::ObjectType::ClassObject(obj::ClassObject::new(obj::ClassObjectData::SkyLightBackground(Default::default()))),
    "CLASSOBJECT_SOLIDBACKGROUND" => obj::ObjectType::ClassObject(obj::ClassObject::new(obj::ClassObjectData::SolidBackground(Default::default()))),
    "CLASSOBJECT_RENDERENTRY" => obj::ObjectType::ClassObject(obj::ClassObject::new(obj::ClassObjectData::RenderEntry(Default::default()))),
    "CLASSOBJECT_RENDERENVIRONMENT" => obj::ObjectType::ClassObject(obj::ClassObject::new(obj::ClassObjectData::RenderEnvironment(Default::default()))),
    "CLASSOBJECT_RENDERGLOBAL" => obj::ObjectType::ClassObject(obj::ClassObject::new(obj::ClassObjectData::RenderGlobal(Default::default()))),
    "CLASSOBJECT_MOTIONPATH" => obj::ObjectType::ClassObject(obj::ClassObject::new(obj::ClassObjectData::MotionPath(Default::default()))),
    "CLASSOBJECT_CURVEPATH" => obj::ObjectType::ClassObject(obj::ClassObject::new(obj::ClassObjectData::CurvePath(Default::default()))),
    "CLASSOBJECT_POINTPATH" => obj::ObjectType::ClassObject(obj::ClassObject::new(obj::ClassObjectData::PointPath(Default::default()))),
    "CLASSOBJECT_TVDEVICEPROPERTIES" => obj::ObjectType::ClassObject(obj::ClassObject::new(obj::ClassObjectData::TvDeviceProperties(Default::default()))),
    "CLASSOBJECT_POINTCLOUDDEFINITION" => obj::ObjectType::ClassObject(obj::ClassObject::new(obj::ClassObjectData::PointCloudDefinition(Default::default()))),
    "CLASSOBJECT_POINTCLOUDDEFINITIONEX" => obj::ObjectType::ClassObject(obj::ClassObject::new(obj::ClassObjectData::PointCloudDefinitionEx(Default::default()))),
    "CLASSOBJECT_POINTCLOUDDEFINITIONREACTOR" => obj::ObjectType::ClassObject(obj::ClassObject::new(obj::ClassObjectData::PointCloudDefinitionReactor(Default::default()))),
    "CLASSOBJECT_POINTCLOUDDEFINITIONREACTOREX" => obj::ObjectType::ClassObject(obj::ClassObject::new(obj::ClassObjectData::PointCloudDefinitionReactorEx(Default::default()))),
    "CLASSOBJECT_POINTCLOUDCOLORMAP" => obj::ObjectType::ClassObject(obj::ClassObject::new(obj::ClassObjectData::PointCloudColorMap(Default::default()))),
    "CLASSOBJECT_NAVISWORKSMODELDEFINITION" => obj::ObjectType::ClassObject(obj::ClassObject::new(obj::ClassObjectData::NavisworksModelDefinition(Default::default()))),
    "CLASSOBJECT_CONTEXTDATAMANAGER" => obj::ObjectType::ClassObject(obj::ClassObject::new(obj::ClassObjectData::ContextDataManager(Default::default()))),
    "CLASSOBJECT_SUNSTUDY" => obj::ObjectType::ClassObject(obj::ClassObject::new(obj::ClassObjectData::SunStudy(Default::default()))),
    "CLASSOBJECT_DATATABLE" => obj::ObjectType::ClassObject(obj::ClassObject::new(obj::ClassObjectData::DataTable(Default::default()))),
    "CLASSOBJECT_DATALINK" => obj::ObjectType::ClassObject(obj::ClassObject::new(obj::ClassObjectData::DataLink(Default::default()))),
    "CLASSOBJECT_PERSISTENTSUBENTITYMANAGER" => obj::ObjectType::ClassObject(obj::ClassObject::new(obj::ClassObjectData::PersistentSubentityManager(Default::default()))),
    "CLASSOBJECT_GEOMAPIMAGE" => obj::ObjectType::ClassObject(obj::ClassObject::new(obj::ClassObjectData::GeoMapImage(Default::default()))),
    "CLASSOBJECT_DETAILVIEWSTYLE" => obj::ObjectType::ClassObject(obj::ClassObject::new(obj::ClassObjectData::DetailViewStyle(Default::default()))),
    "CLASSOBJECT_SECTIONVIEWSTYLE" => obj::ObjectType::ClassObject(obj::ClassObject::new(obj::ClassObjectData::SectionViewStyle(Default::default()))),
    "CLASSOBJECT_ACMECOMMANDHISTORY" => obj::ObjectType::ClassObject(obj::ClassObject::new(obj::ClassObjectData::AcMeCommandHistory(Default::default()))),
    "CLASSOBJECT_ACMESCOPE" => obj::ObjectType::ClassObject(obj::ClassObject::new(obj::ClassObjectData::AcMeScope(Default::default()))),
    "CLASSOBJECT_ACMESTATEMANAGER" => obj::ObjectType::ClassObject(obj::ClassObject::new(obj::ClassObjectData::AcMeStateManager(Default::default()))),
    "CLASSOBJECT_CSACDOCUMENTOPTIONS" => obj::ObjectType::ClassObject(obj::ClassObject::new(obj::ClassObjectData::CsacDocumentOptions(Default::default()))),
    "CLASSOBJECT_VIEWREPSOURCEMANAGER" => obj::ObjectType::ClassObject(obj::ClassObject::new(obj::ClassObjectData::ViewRepSourceManager(Default::default()))),
    "CLASSOBJECT_VIEWREPSTANDARD" => obj::ObjectType::ClassObject(obj::ClassObject::new(obj::ClassObjectData::ViewRepStandard(Default::default()))),
    "CLASSOBJECT_VIEWREPORIENTATIONDEFINITION" => obj::ObjectType::ClassObject(obj::ClassObject::new(obj::ClassObjectData::ViewRepOrientationDefinition)),
    "CLASSOBJECT_VIEWREPORIENTATION" => obj::ObjectType::ClassObject(obj::ClassObject::new(obj::ClassObjectData::ViewRepOrientation(Default::default()))),
    "CLASSOBJECT_VIEWREPSECTIONDEFINITION" => obj::ObjectType::ClassObject(obj::ClassObject::new(obj::ClassObjectData::ViewRepSectionDefinition(Default::default()))),
    "CLASSOBJECT_VIEWREPMODELSPACEVIEWSELECTIONSET" => obj::ObjectType::ClassObject(obj::ClassObject::new(obj::ClassObjectData::ViewRepModelSpaceViewSelectionSet(Default::default()))),
    "CLASSOBJECT_VIEWREP" => obj::ObjectType::ClassObject(obj::ClassObject::new(obj::ClassObjectData::ViewRep(Default::default()))),
    "CLASSOBJECT_VIEWREPMODELSPACESOURCE" => obj::ObjectType::ClassObject(obj::ClassObject::new(obj::ClassObjectData::ViewRepModelSpaceSource(Default::default()))),
    "DATAOBJECT_BREAKPOINTREF" => obj::ObjectType::DataObject(obj::DataObject::new(obj::DataObjectData::BreakPointRef)),
    "DATAOBJECT_BREAKDATA" => obj::ObjectType::DataObject(obj::DataObject::new(obj::DataObjectData::BreakData(Default::default()))),
    "DATAOBJECT_CELLSTYLEMAP" => obj::ObjectType::DataObject(obj::DataObject::new(obj::DataObjectData::CellStyleMap(Default::default()))),
    "DATAOBJECT_ACDSRECORD" => obj::ObjectType::DataObject(obj::DataObject::new(obj::DataObjectData::AcDsRecord)),
    "DATAOBJECT_ACDSSCHEMA" => obj::ObjectType::DataObject(obj::DataObject::new(obj::DataObjectData::AcDsSchema)),
    "DATAOBJECT_DUMMY" => obj::ObjectType::DataObject(obj::DataObject::new(obj::DataObjectData::Dummy)),
    "DATAOBJECT_IDBUFFER" => obj::ObjectType::DataObject(obj::DataObject::new(obj::DataObjectData::IdBuffer(Default::default()))),
    "DATAOBJECT_INDEX" => obj::ObjectType::DataObject(obj::DataObject::new(obj::DataObjectData::Index(Default::default()))),
    "DATAOBJECT_LAYERINDEX" => obj::ObjectType::DataObject(obj::DataObject::new(obj::DataObjectData::LayerIndex(Default::default()))),
    "DATAOBJECT_LONGTRANSACTION" => obj::ObjectType::DataObject(obj::DataObject::new(obj::DataObjectData::LongTransaction)),
    "DATAOBJECT_OBJECTPOINTER" => obj::ObjectType::DataObject(obj::DataObject::new(obj::DataObjectData::ObjectPointer)),
    "DATAOBJECT_PARTIALVIEWINGFILTER" => obj::ObjectType::DataObject(obj::DataObject::new(obj::DataObjectData::PartialViewingFilter(Default::default()))),
    "DATAOBJECT_TABLEGEOMETRY" => obj::ObjectType::DataObject(obj::DataObject::new(obj::DataObjectData::TableGeometry(Default::default()))),
    "DYNAMICBLOCK_UNKNOWN" => dynamic_object(obj::DynamicBlockData::Unknown, "BLOCKUNKNOWN", "AcDbObject"),
    "DYNAMICBLOCK_REPRESENTATION" => dynamic_object(obj::DynamicBlockData::Representation(Default::default()), "ACDB_BLOCKREPRESENTATION_DATA", "AcDbObject"),
    "DYNAMICBLOCK_PROXYNODE" => dynamic_object(obj::DynamicBlockData::ProxyNode(Default::default()), "DYNAMICBLOCKPROXYNODE", "AcDbObject"),
    "DYNAMICBLOCK_GRIPLOCATIONCOMPONENT" => dynamic_object(obj::DynamicBlockData::GripLocationComponent(Default::default()), "BLOCKGRIPLOCATIONCOMPONENT", "AcDbObject"),
    "DYNAMICBLOCK_ALIGNMENTGRIP" => dynamic_object(obj::DynamicBlockData::AlignmentGrip(Default::default()), "BLOCKALIGNMENTGRIP", "AcDbObject"),
    "DYNAMICBLOCK_FLIPGRIP" => dynamic_object(obj::DynamicBlockData::FlipGrip(Default::default()), "BLOCKFLIPGRIP", "AcDbObject"),
    "DYNAMICBLOCK_LINEARGRIP" => dynamic_object(obj::DynamicBlockData::LinearGrip(Default::default()), "BLOCKLINEARGRIP", "AcDbObject"),
    "DYNAMICBLOCK_LOOKUPGRIP" => dynamic_object(obj::DynamicBlockData::LookupGrip(Default::default()), "BLOCKLOOKUPGRIP", "AcDbObject"),
    "DYNAMICBLOCK_POLARGRIP" => dynamic_object(obj::DynamicBlockData::PolarGrip(Default::default()), "BLOCKPOLARGRIP", "AcDbObject"),
    "DYNAMICBLOCK_ROTATIONGRIP" => dynamic_object(obj::DynamicBlockData::RotationGrip(Default::default()), "BLOCKROTATIONGRIP", "AcDbObject"),
    "DYNAMICBLOCK_VISIBILITYGRIP" => dynamic_object(obj::DynamicBlockData::VisibilityGrip(Default::default()), "BLOCKVISIBILITYGRIP", "AcDbObject"),
    "DYNAMICBLOCK_XYGRIP" => dynamic_object(obj::DynamicBlockData::XYGrip(Default::default()), "BLOCKXYGRIP", "AcDbObject"),
    "DYNAMICBLOCK_PROPERTIESTABLEGRIP" => dynamic_object(obj::DynamicBlockData::PropertiesTableGrip(Default::default()), "BLOCKPROPERTIESTABLEGRIP", "AcDbObject"),
    "DYNAMICBLOCK_ALIGNMENTPARAMETER" => dynamic_object(obj::DynamicBlockData::AlignmentParameter(Default::default()), "BLOCKALIGNMENTPARAMETER", "AcDbObject"),
    "DYNAMICBLOCK_BASEPOINTPARAMETER" => dynamic_object(obj::DynamicBlockData::BasePointParameter(Default::default()), "BLOCKBASEPOINTPARAMETER", "AcDbObject"),
    "DYNAMICBLOCK_FLIPPARAMETER" => dynamic_object(obj::DynamicBlockData::FlipParameter(Default::default()), "BLOCKFLIPPARAMETER", "AcDbObject"),
    "DYNAMICBLOCK_LINEARPARAMETER" => dynamic_object(obj::DynamicBlockData::LinearParameter(Default::default()), "BLOCKLINEARPARAMETER", "AcDbObject"),
    "DYNAMICBLOCK_LOOKUPPARAMETER" => dynamic_object(obj::DynamicBlockData::LookupParameter(Default::default()), "BLOCKLOOKUPPARAMETER", "AcDbObject"),
    "DYNAMICBLOCK_POINTPARAMETER" => dynamic_object(obj::DynamicBlockData::PointParameter(Default::default()), "BLOCKPOINTPARAMETER", "AcDbObject"),
    "DYNAMICBLOCK_POLARPARAMETER" => dynamic_object(obj::DynamicBlockData::PolarParameter(Default::default()), "BLOCKPOLARPARAMETER", "AcDbObject"),
    "DYNAMICBLOCK_ROTATIONPARAMETER" => dynamic_object(obj::DynamicBlockData::RotationParameter(Default::default()), "BLOCKROTATIONPARAMETER", "AcDbObject"),
    "DYNAMICBLOCK_USERPARAMETER" => dynamic_object(obj::DynamicBlockData::UserParameter(Default::default()), "BLOCKUSERPARAMETER", "AcDbObject"),
    "DYNAMICBLOCK_VISIBILITYPARAMETER" => dynamic_object(obj::DynamicBlockData::VisibilityParameter(Default::default()), "BLOCKVISIBILITYPARAMETER", "AcDbObject"),
    "DYNAMICBLOCK_XYPARAMETER" => dynamic_object(obj::DynamicBlockData::XYParameter(Default::default()), "BLOCKXYPARAMETER", "AcDbObject"),
    "DYNAMICBLOCK_ANGULARCONSTRAINTPARAMETER" => dynamic_object(obj::DynamicBlockData::AngularConstraintParameter(Default::default()), "BLOCKANGULARCONSTRAINTPARAMETER", "AcDbObject"),
    "DYNAMICBLOCK_DIAMETRICCONSTRAINTPARAMETER" => dynamic_object(obj::DynamicBlockData::DiametricConstraintParameter(Default::default()), "BLOCKDIAMETRICCONSTRAINTPARAMETER", "AcDbObject"),
    "DYNAMICBLOCK_RADIALCONSTRAINTPARAMETER" => dynamic_object(obj::DynamicBlockData::RadialConstraintParameter(Default::default()), "BLOCKRADIALCONSTRAINTPARAMETER", "AcDbObject"),
    "DYNAMICBLOCK_ALIGNEDCONSTRAINTPARAMETER" => dynamic_object(obj::DynamicBlockData::AlignedConstraintParameter(Default::default()), "BLOCKALIGNEDCONSTRAINTPARAMETER", "AcDbObject"),
    "DYNAMICBLOCK_LINEARCONSTRAINTPARAMETER" => dynamic_object(obj::DynamicBlockData::LinearConstraintParameter(Default::default()), "BLOCKLINEARCONSTRAINTPARAMETER", "AcDbObject"),
    "DYNAMICBLOCK_HORIZONTALCONSTRAINTPARAMETER" => dynamic_object(obj::DynamicBlockData::HorizontalConstraintParameter(Default::default()), "BLOCKHORIZONTALCONSTRAINTPARAMETER", "AcDbObject"),
    "DYNAMICBLOCK_VERTICALCONSTRAINTPARAMETER" => dynamic_object(obj::DynamicBlockData::VerticalConstraintParameter(Default::default()), "BLOCKVERTICALCONSTRAINTPARAMETER", "AcDbObject"),
    "DYNAMICBLOCK_PARAMETERDEPENDENCYBODY" => dynamic_object(obj::DynamicBlockData::ParameterDependencyBody(Default::default()), "ACDBBLOCKPARAMDEPENDENCYBODY", "AcDbObject"),
    "DYNAMICBLOCK_MOVEACTION" => dynamic_object(obj::DynamicBlockData::MoveAction(Default::default()), "BLOCKMOVEACTION", "AcDbObject"),
    "DYNAMICBLOCK_FLIPACTION" => dynamic_object(obj::DynamicBlockData::FlipAction(Default::default()), "BLOCKFLIPACTION", "AcDbObject"),
    "DYNAMICBLOCK_ROTATEACTION" => dynamic_object(obj::DynamicBlockData::RotateAction(Default::default()), "BLOCKROTATEACTION", "AcDbObject"),
    "DYNAMICBLOCK_SCALEACTION" => dynamic_object(obj::DynamicBlockData::ScaleAction(Default::default()), "BLOCKSCALEACTION", "AcDbObject"),
    "DYNAMICBLOCK_ARRAYACTION" => dynamic_object(obj::DynamicBlockData::ArrayAction(Default::default()), "BLOCKARRAYACTION", "AcDbObject"),
    "DYNAMICBLOCK_LOOKUPACTION" => dynamic_object(obj::DynamicBlockData::LookupAction(Default::default()), "BLOCKLOOKUPACTION", "AcDbObject"),
    "DYNAMICBLOCK_STRETCHACTION" => dynamic_object(obj::DynamicBlockData::StretchAction(Default::default()), "BLOCKSTRETCHACTION", "AcDbObject"),
    "DYNAMICBLOCK_POLARSTRETCHACTION" => dynamic_object(obj::DynamicBlockData::PolarStretchAction(Default::default()), "BLOCKPOLARSTRETCHACTION", "AcDbObject"),
    "DYNAMICBLOCK_ALIGNMENTPARAMETERENTITY" => dynamic_object(obj::DynamicBlockData::AlignmentParameterEntity, "BLOCKALIGNMENTPARAMETERENTITY", "AcDbObject"),
    "DYNAMICBLOCK_BASEPOINTPARAMETERENTITY" => dynamic_object(obj::DynamicBlockData::BasePointParameterEntity, "BLOCKBASEPOINTPARAMETERENTITY", "AcDbObject"),
    "DYNAMICBLOCK_FLIPPARAMETERENTITY" => dynamic_object(obj::DynamicBlockData::FlipParameterEntity, "BLOCKFLIPPARAMETERENTITY", "AcDbObject"),
    "DYNAMICBLOCK_LINEARPARAMETERENTITY" => dynamic_object(obj::DynamicBlockData::LinearParameterEntity, "BLOCKLINEARPARAMETERENTITY", "AcDbObject"),
    "DYNAMICBLOCK_POINTPARAMETERENTITY" => dynamic_object(obj::DynamicBlockData::PointParameterEntity, "BLOCKPOINTPARAMETERENTITY", "AcDbObject"),
    "DYNAMICBLOCK_ROTATIONPARAMETERENTITY" => dynamic_object(obj::DynamicBlockData::RotationParameterEntity, "BLOCKROTATIONPARAMETERENTITY", "AcDbObject"),
    "DYNAMICBLOCK_VISIBILITYPARAMETERENTITY" => dynamic_object(obj::DynamicBlockData::VisibilityParameterEntity, "BLOCKVISIBILITYPARAMETERENTITY", "AcDbObject"),
    "DYNAMICBLOCK_XYPARAMETERENTITY" => dynamic_object(obj::DynamicBlockData::XYParameterEntity, "BLOCKXYPARAMETERENTITY", "AcDbObject"),
    "DYNAMICBLOCK_ANGULARCONSTRAINTPARAMETERENTITY" => dynamic_object(obj::DynamicBlockData::AngularConstraintParameterEntity(Default::default()), "BLOCKANGULARCONSTRAINTPARAMETERENTITY", "AcDbObject"),
    "DYNAMICBLOCK_FLIPGRIPENTITY" => dynamic_object(obj::DynamicBlockData::FlipGripEntity, "BLOCKFLIPGRIPENTITY", "AcDbObject"),
    "DYNAMICBLOCK_LINEARGRIPENTITY" => dynamic_object(obj::DynamicBlockData::LinearGripEntity, "BLOCKLINEARGRIPENTITY", "AcDbObject"),
    "DYNAMICBLOCK_POLARGRIPENTITY" => dynamic_object(obj::DynamicBlockData::PolarGripEntity, "BLOCKPOLARGRIPENTITY", "AcDbObject"),
    "DYNAMICBLOCK_ROTATIONGRIPENTITY" => dynamic_object(obj::DynamicBlockData::RotationGripEntity, "BLOCKROTATIONGRIPENTITY", "AcDbObject"),
    "DYNAMICBLOCK_VISIBILITYGRIPENTITY" => dynamic_object(obj::DynamicBlockData::VisibilityGripEntity, "BLOCKVISIBILITYGRIPENTITY", "AcDbObject"),
    "DYNAMICBLOCK_XYGRIPENTITY" => dynamic_object(obj::DynamicBlockData::XYGripEntity, "BLOCKXYGRIPENTITY", "AcDbObject"),
    "DYNAMICBLOCK_EVALUATIONGRAPH" => dynamic_object(obj::DynamicBlockData::EvaluationGraph(Default::default()), "EVALUATION_GRAPH", "AcDbObject"),
    "DYNAMICBLOCK_SOLIDHISTORY" => dynamic_object(obj::DynamicBlockData::SolidHistory(Default::default()), "BLOCKSOLIDHISTORY", "AcDbObject"),
    "DYNAMICBLOCK_SOLIDHISTORYNODE" => dynamic_object(obj::DynamicBlockData::SolidHistoryNode(Default::default()), "BLOCKSOLIDHISTORYNODE", "AcDbObject"),
    "ASSOCIATIVE_UNKNOWN" => associative_object(obj::AssociativeData::Unknown, "ASSOCUNKNOWN"),
    "ASSOCIATIVE_DEPENDENCY" => associative_object(obj::AssociativeData::Dependency(Default::default()), "ASSOCDEPENDENCY"),
    "ASSOCIATIVE_VALUEDEPENDENCY" => associative_object(obj::AssociativeData::ValueDependency(Default::default()), "ASSOCVALUEDEPENDENCY"),
    "ASSOCIATIVE_GEOMDEPENDENCY" => associative_object(obj::AssociativeData::GeomDependency(Default::default()), "ASSOCGEOMDEPENDENCY"),
    "ASSOCIATIVE_SURFACEACTIONBODY" => associative_object(obj::AssociativeData::SurfaceActionBody(Default::default()), "ASSOCSURFACEACTIONBODY"),
    "ASSOCIATIVE_ACTION" => associative_object(obj::AssociativeData::Action(Default::default()), "ASSOCACTION"),
    "ASSOCIATIVE_NETWORK" => associative_object(obj::AssociativeData::Network(Default::default()), "ASSOCNETWORK"),
    "ASSOCIATIVE_ANNOTATIONACTIONBODY" => associative_object(obj::AssociativeData::AnnotationActionBody(Default::default()), "ASSOCRESTOREENTITYSTATEACTIONBODY"),
    "ASSOCIATIVE_PERSSUBENTMANAGER" => associative_object(obj::AssociativeData::PersSubentManager(Default::default()), "ASSOCPERSSUBENTMANAGER"),
    "ASSOCIATIVE_EDGEACTIONPARAM" => associative_object(obj::AssociativeData::EdgeActionParam(Default::default()), "ASSOCEDGEACTIONPARAM"),
    "ASSOCIATIVE_CONSTRAINTGROUP" => associative_object(obj::AssociativeData::ConstraintGroup(Default::default()), "ASSOC2DCONSTRAINTGROUP"),
    "ASSOCIATIVE_VARIABLE" => associative_object(obj::AssociativeData::Variable(Default::default()), "ASSOCVARIABLE"),
    "ASSOCIATIVE_ACTIONPARAM" => associative_object(obj::AssociativeData::ActionParam(Default::default()), "ASSOCACTIONPARAM"),
    "ASSOCIATIVE_COMPOUNDACTIONPARAM" => associative_object(obj::AssociativeData::CompoundActionParam(Default::default()), "ASSOCCOMPOUNDACTIONPARAM"),
    "ASSOCIATIVE_OSNAPPOINTREFACTIONPARAM" => associative_object(obj::AssociativeData::OsnapPointRefActionParam(Default::default()), "ASSOCOSNAPPOINTREFACTIONPARAM"),
    "ASSOCIATIVE_POINTREFACTIONPARAM" => associative_object(obj::AssociativeData::PointRefActionParam(Default::default()), "ASSOCPOINTREFACTIONPARAM"),
    "ASSOCIATIVE_OBJECTACTIONPARAM" => associative_object(obj::AssociativeData::ObjectActionParam(Default::default()), "ASSOCOBJECTACTIONPARAM"),
    "ASSOCIATIVE_PATHACTIONPARAM" => associative_object(obj::AssociativeData::PathActionParam(Default::default()), "ASSOCPATHACTIONPARAM"),
    "ASSOCIATIVE_DIMDEPENDENCYBODY" => associative_object(obj::AssociativeData::DimDependencyBody(Default::default()), "ASSOCDIMDEPENDENCYBODY"),
    "ASSOCIATIVE_FACEACTIONPARAM" => associative_object(obj::AssociativeData::FaceActionParam(Default::default()), "ASSOCOBJECTACTIONPARAM"),
    "ASSOCIATIVE_VERTEXACTIONPARAM" => associative_object(obj::AssociativeData::VertexActionParam(Default::default()), "ASSOCVERTEXACTIONPARAM"),
    "ASSOCIATIVE_ASMBODYACTIONPARAM" => associative_object(obj::AssociativeData::AsmBodyActionParam(Default::default()), "ASSOCASMBODYACTIONPARAM"),
    "ASSOCIATIVE_ARRAYPARAMETERS" => associative_object(obj::AssociativeData::ArrayParameters(Default::default()), "ASSOCARRAYMODIFYPARAMETERS"),
    "ASSOCIATIVE_ARRAYACTIONBODY" => associative_object(obj::AssociativeData::ArrayActionBody(Default::default()), "ASSOCARRAYACTIONBODY"),
    "ASSOCIATIVE_ARRAYMODIFYACTIONBODY" => associative_object(obj::AssociativeData::ArrayModifyActionBody(Default::default()), "ASSOCARRAYACTIONBODY"),
    "ASSOCIATIVE_DIMENSIONASSOCIATION" => associative_object(obj::AssociativeData::DimensionAssociation(Default::default()), "DIMASSOC"),
    "ASSOCIATIVE_PERSSUBENTMANAGERSTATIC" => associative_object(obj::AssociativeData::PersSubentManagerStatic(Default::default()), "PERSUBENTMGR"),
    "ASSOCIATIVE_VIEWREPACTIONBODY" => associative_object(obj::AssociativeData::ViewRepActionBody(Default::default()), "ASSOCVIEWREPACTIONBODY"),
    "ASSOCIATIVE_VIEWOBJECTACTIONPARAM" => associative_object(obj::AssociativeData::ViewObjectActionParam(Default::default()), "ASSOCVIEWBORDERACTIONPARAM"),
    "ASSOCIATIVE_VIEWREPHATCHMANAGER" => associative_object(obj::AssociativeData::ViewRepHatchManager(Default::default()), "ASSOCVIEWREPHATCHMANAGER"),
    "ASSOCIATIVE_VIEWREPHATCHACTIONPARAM" => associative_object(obj::AssociativeData::ViewRepHatchActionParam(Default::default()), "ASSOCVIEWREPHATCHACTIONPARAM"),
    "ASSOCIATIVE_VIEWLABELACTIONPARAM" => associative_object(obj::AssociativeData::ViewLabelActionParam(Default::default()), "ASSOCVIEWLABELACTIONPARAM"),
    "ASSOCIATIVE_SMARTCENTERACTIONBODY" => associative_object(obj::AssociativeData::SmartCenterActionBody(Default::default()), "ACDBCENTERMARKACTIONBODY"),
    "SOLIDHISTORY_UNKNOWN" => history_object(obj::SolidHistoryOperation::Unknown),
    "SOLIDHISTORY_BOX" => history_object(obj::SolidHistoryOperation::Box(Default::default())),
    "SOLIDHISTORY_WEDGE" => history_object(obj::SolidHistoryOperation::Wedge(Default::default())),
    "SOLIDHISTORY_SPHERE" => history_object(obj::SolidHistoryOperation::Sphere(Default::default())),
    "SOLIDHISTORY_CYLINDER" => history_object(obj::SolidHistoryOperation::Cylinder(Default::default())),
    "SOLIDHISTORY_CONE" => history_object(obj::SolidHistoryOperation::Cone(Default::default())),
    "SOLIDHISTORY_PYRAMID" => history_object(obj::SolidHistoryOperation::Pyramid(Default::default())),
    "SOLIDHISTORY_TORUS" => history_object(obj::SolidHistoryOperation::Torus(Default::default())),
    "SOLIDHISTORY_BOOLEAN" => history_object(obj::SolidHistoryOperation::Boolean(Default::default())),
    "SOLIDHISTORY_BREP" => history_object(obj::SolidHistoryOperation::Brep(Default::default())),
    "SOLIDHISTORY_FILLET" => history_object(obj::SolidHistoryOperation::Fillet(Default::default())),
    "SOLIDHISTORY_CHAMFER" => history_object(obj::SolidHistoryOperation::Chamfer(Default::default())),
    "SOLIDHISTORY_SWEEP" => history_object(obj::SolidHistoryOperation::Sweep(Default::default())),
    "SOLIDHISTORY_EXTRUSION" => history_object(obj::SolidHistoryOperation::Extrusion(Default::default())),
    "SOLIDHISTORY_LOFT" => history_object(obj::SolidHistoryOperation::Loft(Default::default())),
    "SOLIDHISTORY_REVOLVE" => history_object(obj::SolidHistoryOperation::Revolve(Default::default())),
    "CONTEXT_ANNOTSCALE" => context_object(obj::ObjectContextKind::AnnotScale),
    "CONTEXT_BLKREF" => context_object(obj::ObjectContextKind::BlkRef { rotation: Default::default(), insertion: Default::default(), scale_factor: Default::default() }),
    "CONTEXT_TEXT" => context_object(obj::ObjectContextKind::Text { horizontal_mode: Default::default(), rotation: Default::default(), insertion: Default::default(), alignment: Default::default() }),
    "CONTEXT_MTEXT" => context_object(obj::ObjectContextKind::MText(obj::MTextContext { attachment: Default::default(), x_axis_dir: Default::default(), insertion: Default::default(), rect_width: Default::default(), rect_height: Default::default(), extents_width: Default::default(), extents_height: Default::default(), column_type: Default::default(), columns: Default::default() })),
    "CONTEXT_DIM" => context_object(obj::ObjectContextKind::Dim(obj::DimContext { def_pt: Default::default(), is_def_textloc: Default::default(), text_rotation: Default::default(), block: Default::default(), b293: Default::default(), dimtofl: Default::default(), dimosxd: Default::default(), dimatfit: Default::default(), dimtix: Default::default(), dimtmove: Default::default(), override_code: Default::default(), has_arrow2: Default::default(), flip_arrow2: Default::default(), flip_arrow1: Default::default(), subtype: obj::DimSubtype::Aligned { dimline_pt: Default::default() } })),
    "CONTEXT_MLEADER" => context_object(obj::ObjectContextKind::MLeader(Default::default())),
    "CONTEXT_MTEXTATTRIBUTE" => context_object(obj::ObjectContextKind::MTextAttribute(obj::MTextAttributeContext { horizontal_mode: Default::default(), rotation: Default::default(), insertion: Default::default(), alignment: Default::default(), enable_context: Default::default(), context: Default::default() })),
    "CONTEXT_LEADER" => context_object(obj::ObjectContextKind::Leader(obj::LeaderContext { points: Default::default(), x_direction: Default::default(), annotation_enabled: Default::default(), insertion_offset: Default::default(), endpoint_projection: Default::default() })),
    "CONTEXT_FCF" => context_object(obj::ObjectContextKind::Fcf { location: Default::default(), horizontal_direction: Default::default() }),
    "CONTEXT_HATCHSCALE" => context_object(obj::ObjectContextKind::HatchScale(obj::HatchScaleContext { pattern_lines: Default::default(), pattern_scale: Default::default(), pattern_base: Default::default(), loops: Default::default() })),
    "CONTEXT_HATCHVIEW" => context_object(obj::ObjectContextKind::HatchView(obj::HatchViewContext { hatch: obj::HatchScaleContext { pattern_lines: Default::default(), pattern_scale: Default::default(), pattern_base: Default::default(), loops: Default::default() }, view: Default::default(), view_normal: Default::default(), view_rotation: Default::default(), evaluate_hatch: Default::default() })),
    "CONTEXT_OPAQUE" => context_object(obj::ObjectContextKind::Opaque),
    "DGNLINESTYLE_DEFINITION" => dgn_object(obj::DgnLineStyleData::Definition { description: Default::default(), version: Default::default(), style_number: Default::default(), component_uid: Default::default(), is_continuous: Default::default(), unit_definition: Default::default(), unit_scale: Default::default(), units_type: Default::default(), is_element: Default::default(), is_physical: Default::default(), is_scale_independent: Default::default(), is_snappable: Default::default(), root_component: Default::default(), properties: Default::default() }),
    "DGNLINESTYLE_COMPONENT" => dgn_object(obj::DgnLineStyleData::Component { kind: obj::DgnLsComponentType::Symbol, description: Default::default(), version: Default::default(), component_uid: Default::default(), scale: Default::default(), property_flags: Default::default(), component: obj::DgnLsComponentData::Symbol(obj::DgnLsSymbolComponent { stored_unit_scale: Default::default(), unit_scale: Default::default(), has_unit_scale: Default::default(), is_3d: Default::default(), block: Default::default() }), properties: Default::default() }),
    "DGNLINESTYLE_REGISTERED" => dgn_object(obj::DgnLineStyleData::Registered { dxf_name: Default::default(), properties: Default::default(), payload: Default::default(), object_ids: Default::default() }),
    "DGNCOMPONENT_SYMBOL" => dgn_component(obj::DgnLsComponentData::Symbol(obj::DgnLsSymbolComponent { stored_unit_scale: Default::default(), unit_scale: Default::default(), has_unit_scale: Default::default(), is_3d: Default::default(), block: Default::default() })),
    "DGNCOMPONENT_COMPOUND" => dgn_component(obj::DgnLsComponentData::Compound(obj::DgnLsCompoundComponent { entries: Default::default() })),
    "DGNCOMPONENT_STROKE" => dgn_component(obj::DgnLsComponentData::Stroke(obj::DgnLsStrokePattern { has_iteration_limit: Default::default(), is_single_segment: Default::default(), iteration_limit: Default::default(), auto_phase: Default::default(), phase: Default::default(), phase_mode: obj::DgnLsPhaseMode::Distance, strokes: Default::default() })),
    "DGNCOMPONENT_POINT" => dgn_component(obj::DgnLsComponentData::Point(obj::DgnLsPointComponent { stroke_component: Default::default(), symbols: Default::default() })),
    "DGNCOMPONENT_INTERNAL" => dgn_component(obj::DgnLsComponentData::Internal(obj::DgnLsInternalComponent { pattern: obj::DgnLsStrokePattern { has_iteration_limit: Default::default(), is_single_segment: Default::default(), iteration_limit: Default::default(), auto_phase: Default::default(), phase: Default::default(), phase_mode: obj::DgnLsPhaseMode::Distance, strokes: Default::default() }, internal_version: Default::default(), hardware_style: Default::default(), is_hardware_style: Default::default(), line_code: Default::default() })),
    "ASSOCIATIVE_SURFACE_PLANE" => surface_action(obj::AssocSurfaceActionKind::Plane),
    "ASSOCIATIVE_SURFACE_EXTEND" => surface_action(obj::AssocSurfaceActionKind::Extend),
    "ASSOCIATIVE_SURFACE_EXTRUDED" => surface_action(obj::AssocSurfaceActionKind::Extruded),
    "ASSOCIATIVE_SURFACE_LOFTED" => surface_action(obj::AssocSurfaceActionKind::Lofted),
    "ASSOCIATIVE_SURFACE_NETWORK" => surface_action(obj::AssocSurfaceActionKind::Network),
    "ASSOCIATIVE_SURFACE_OFFSET" => surface_action(obj::AssocSurfaceActionKind::Offset),
    "ASSOCIATIVE_SURFACE_REVOLVED" => surface_action(obj::AssocSurfaceActionKind::Revolved),
    "ASSOCIATIVE_SURFACE_TRIM" => surface_action(obj::AssocSurfaceActionKind::Trim),
    "ASSOCIATIVE_SURFACE_BLEND" => surface_action(obj::AssocSurfaceActionKind::Blend),
    "ASSOCIATIVE_SURFACE_PATCH" => surface_action(obj::AssocSurfaceActionKind::Patch),
    "ASSOCIATIVE_SURFACE_FILLET" => surface_action(obj::AssocSurfaceActionKind::Fillet),
    "ASSOCIATIVE_SURFACE_SWEPT" => surface_action(obj::AssocSurfaceActionKind::Swept),
    "ASSOCIATIVE_SURFACE_EDGECHAMFER" => surface_action(obj::AssocSurfaceActionKind::EdgeChamfer),
    "ASSOCIATIVE_SURFACE_EDGEFILLET" => surface_action(obj::AssocSurfaceActionKind::EdgeFillet),
    "ASSOCIATIVE_ANNOTATION_RESTOREENTITYSTATE" => annotation_action(obj::AssocAnnotationKind::RestoreEntityState),
    "ASSOCIATIVE_ANNOTATION_MLEADER" => annotation_action(obj::AssocAnnotationKind::MLeader),
    "ASSOCIATIVE_ANNOTATION_ALIGNEDDIMENSION" => annotation_action(obj::AssocAnnotationKind::AlignedDimension),
    "ASSOCIATIVE_ANNOTATION_THREEPOINTANGULARDIMENSION" => annotation_action(obj::AssocAnnotationKind::ThreePointAngularDimension),
    "ASSOCIATIVE_ANNOTATION_ORDINATEDIMENSION" => annotation_action(obj::AssocAnnotationKind::OrdinateDimension),
    "ASSOCIATIVE_ANNOTATION_ROTATEDDIMENSION" => annotation_action(obj::AssocAnnotationKind::RotatedDimension),
    "ASSOCIATIVE_VIEWPARAM_VIEWBORDER" => view_action_parameter(obj::AssocViewObjectActionParamKind::ViewBorder),
    "ASSOCIATIVE_VIEWPARAM_VIEWREP" => view_action_parameter(obj::AssocViewObjectActionParamKind::ViewRep),
    "ASSOCIATIVE_VIEWPARAM_VIEWSYMBOL" => view_action_parameter(obj::AssocViewObjectActionParamKind::ViewSymbol),
    "ASSOCIATIVE_VIEWPARAM_VIEWSTYLE" => view_action_parameter(obj::AssocViewObjectActionParamKind::ViewStyle),
    "CONTEXT_DIM_ALIGNED" => dimension_context(obj::DimSubtype::Aligned { dimline_pt: Default::default() }),
    "CONTEXT_DIM_ANGULAR" => dimension_context(obj::DimSubtype::Angular { arc_pt: Default::default() }),
    "CONTEXT_DIM_DIAMETRIC" => dimension_context(obj::DimSubtype::Diametric { first_arc_pt: Default::default(), def_pt: Default::default() }),
    "CONTEXT_DIM_RADIAL" => dimension_context(obj::DimSubtype::Radial { first_arc_pt: Default::default() }),
    "CONTEXT_DIM_RADIALLARGE" => dimension_context(obj::DimSubtype::RadialLarge { ovr_center: Default::default(), jog_point: Default::default() }),
    "CONTEXT_DIM_ORDINATE" => dimension_context(obj::DimSubtype::Ordinate { feature_location_pt: Default::default(), leader_endpt: Default::default() }),
    "ASSOCARRAYMODIFYPARAMETERS" => array_parameters("ASSOCARRAYMODIFYPARAMETERS"),
    "ASSOCARRAYPATHPARAMETERS" => array_parameters("ASSOCARRAYPATHPARAMETERS"),
    "ASSOCARRAYPOLARPARAMETERS" => array_parameters("ASSOCARRAYPOLARPARAMETERS"),
    "ASSOCARRAYRECTANGULARPARAMETERS" => array_parameters("ASSOCARRAYRECTANGULARPARAMETERS"),
    "PDFDEFINITION" => obj::ObjectType::UnderlayDefinition(obj::UnderlayDefinition::new(acadrust::entities::UnderlayType::Pdf)),
    "DWFDEFINITION" => obj::ObjectType::UnderlayDefinition(obj::UnderlayDefinition::new(acadrust::entities::UnderlayType::Dwf)),
    "DGNDEFINITION" => obj::ObjectType::UnderlayDefinition(obj::UnderlayDefinition::new(acadrust::entities::UnderlayType::Dgn)),
}
fn set_identity(value: &mut obj::ObjectType, handle: Handle, owner: Handle) {
    match value {
        obj::ObjectType::Dictionary(v) => {
            v.handle = handle;
            v.owner = owner;
        }
        obj::ObjectType::Layout(v) => {
            v.handle = handle;
            v.owner = owner;
        }
        obj::ObjectType::XRecord(v) => {
            v.handle = handle;
            v.owner = owner;
        }
        obj::ObjectType::Group(v) => {
            v.handle = handle;
            v.owner = owner;
        }
        obj::ObjectType::MLineStyle(v) => {
            v.handle = handle;
            v.owner = owner;
        }
        obj::ObjectType::ImageDefinition(v) => {
            v.handle = handle;
            v.owner = owner;
        }
        obj::ObjectType::UnderlayDefinition(v) => {
            v.handle = handle;
            v.owner_handle = owner;
        }
        obj::ObjectType::PlotSettings(v) => {
            v.handle = handle;
            v.owner = owner;
        }
        obj::ObjectType::MultiLeaderStyle(v) => {
            v.handle = handle;
            v.owner_handle = owner;
        }
        obj::ObjectType::TableStyle(v) => {
            v.handle = handle;
            v.owner_handle = owner;
        }
        obj::ObjectType::TableContent(v) => {
            v.common.handle = handle;
            v.common.owner_handle = owner;
        }
        obj::ObjectType::Scale(v) => {
            v.handle = handle;
            v.owner_handle = owner;
        }
        obj::ObjectType::ObjectContextData(v) => {
            v.handle = handle;
            v.owner_handle = owner;
        }
        obj::ObjectType::SortEntitiesTable(v) => {
            v.handle = handle;
            v.owner_handle = owner;
        }
        obj::ObjectType::DictionaryVariable(v) => {
            v.handle = handle;
            v.owner_handle = owner;
        }
        obj::ObjectType::VisualStyle(v) => {
            v.handle = handle;
            v.owner = owner;
        }
        obj::ObjectType::Material(v) => {
            v.handle = handle;
            v.owner = owner;
        }
        obj::ObjectType::ImageDefinitionReactor(v) => {
            v.handle = handle;
            v.owner = owner;
        }
        obj::ObjectType::GeoData(v) => {
            v.handle = handle;
            v.owner = owner;
        }
        obj::ObjectType::SpatialFilter(v) => {
            v.handle = handle;
            v.owner = owner;
        }
        obj::ObjectType::RasterVariables(v) => {
            v.handle = handle;
            v.owner = owner;
        }
        obj::ObjectType::BookColor(v) => {
            v.handle = handle;
            v.owner = owner;
        }
        obj::ObjectType::PlaceHolder(v) => {
            v.handle = handle;
            v.owner = owner;
        }
        obj::ObjectType::DictionaryWithDefault(v) => {
            v.handle = handle;
            v.owner = owner;
        }
        obj::ObjectType::WipeoutVariables(v) => {
            v.handle = handle;
            v.owner = owner;
        }
        obj::ObjectType::BlockVisibilityParameter(v) => {
            v.handle = handle;
            v.owner = owner;
        }
        obj::ObjectType::DynamicBlock(v) => {
            v.handle = handle;
            v.owner = owner;
        }
        obj::ObjectType::Associative(v) => {
            v.handle = handle;
            v.owner = owner;
        }
        obj::ObjectType::ClassObject(v) => {
            v.handle = handle;
            v.owner = owner;
        }
        obj::ObjectType::DataObject(v) => {
            v.handle = handle;
            v.owner = owner;
        }
        obj::ObjectType::Field(v) => {
            v.handle = handle;
            v.owner = owner;
        }
        obj::ObjectType::FieldList(v) => {
            v.handle = handle;
            v.owner = owner;
        }
        obj::ObjectType::RegisteredClass(v) => {
            v.handle = handle;
            v.owner = owner;
        }
        obj::ObjectType::DgnLineStyle(v) => {
            v.handle = handle;
            v.owner = owner;
        }
        obj::ObjectType::ProxyObject(v) => {
            v.handle = handle;
            v.owner = owner;
        }
        obj::ObjectType::Unknown {
            handle: h,
            owner: o,
            ..
        } => {
            *h = handle;
            *o = owner;
        }
    }
}

fn clear_clone_links(value: &mut obj::ObjectType) {
    match value {
        obj::ObjectType::Dictionary(v) => {
            v.reactors.clear();
            v.xdictionary_handle = None;
        }
        obj::ObjectType::Layout(v) => {
            v.reactors.clear();
            v.xdictionary_handle = None;
        }
        obj::ObjectType::XRecord(v) => {
            v.reactors.clear();
            v.xdictionary_handle = None;
        }
        obj::ObjectType::PlotSettings(v) => {
            v.reactors.clear();
            v.xdictionary_handle = None;
        }
        obj::ObjectType::TableContent(v) => {
            v.common.reactors.clear();
            v.common.xdictionary_handle = None;
        }
        obj::ObjectType::ObjectContextData(v) => {
            v.reactors.clear();
            v.xdictionary_handle = None;
        }
        obj::ObjectType::VisualStyle(v) => {
            v.reactors.clear();
            v.xdictionary_handle = None;
        }
        obj::ObjectType::Material(v) => {
            v.reactors.clear();
            v.xdictionary_handle = None;
        }
        obj::ObjectType::GeoData(v) => {
            v.reactors.clear();
            v.xdictionary_handle = None;
        }
        obj::ObjectType::DynamicBlock(v) => {
            v.reactors.clear();
            v.xdictionary_handle = None;
        }
        obj::ObjectType::Associative(v) => {
            v.reactors.clear();
            v.xdictionary_handle = None;
        }
        obj::ObjectType::ClassObject(v) => {
            v.reactors.clear();
            v.xdictionary_handle = None;
        }
        obj::ObjectType::DataObject(v) => {
            v.reactors.clear();
            v.xdictionary_handle = None;
        }
        obj::ObjectType::RegisteredClass(v) => {
            v.reactors.clear();
            v.xdictionary_handle = None;
        }
        obj::ObjectType::DgnLineStyle(v) => {
            v.reactors.clear();
            v.xdictionary_handle = None;
        }
        obj::ObjectType::ProxyObject(v) => {
            v.reactors.clear();
            v.xdictionary_handle = None;
        }
        _ => {}
    }
}

fn object_owner(value: &obj::ObjectType) -> Handle {
    match value {
        obj::ObjectType::Dictionary(v) => v.owner,
        obj::ObjectType::Layout(v) => v.owner,
        obj::ObjectType::XRecord(v) => v.owner,
        obj::ObjectType::Group(v) => v.owner,
        obj::ObjectType::MLineStyle(v) => v.owner,
        obj::ObjectType::ImageDefinition(v) => v.owner,
        obj::ObjectType::UnderlayDefinition(v) => v.owner_handle,
        obj::ObjectType::PlotSettings(v) => v.owner,
        obj::ObjectType::MultiLeaderStyle(v) => v.owner_handle,
        obj::ObjectType::TableStyle(v) => v.owner_handle,
        obj::ObjectType::TableContent(v) => v.common.owner_handle,
        obj::ObjectType::Scale(v) => v.owner_handle,
        obj::ObjectType::ObjectContextData(v) => v.owner_handle,
        obj::ObjectType::SortEntitiesTable(v) => v.owner_handle,
        obj::ObjectType::DictionaryVariable(v) => v.owner_handle,
        obj::ObjectType::VisualStyle(v) => v.owner,
        obj::ObjectType::Material(v) => v.owner,
        obj::ObjectType::ImageDefinitionReactor(v) => v.owner,
        obj::ObjectType::GeoData(v) => v.owner,
        obj::ObjectType::SpatialFilter(v) => v.owner,
        obj::ObjectType::RasterVariables(v) => v.owner,
        obj::ObjectType::BookColor(v) => v.owner,
        obj::ObjectType::PlaceHolder(v) => v.owner,
        obj::ObjectType::DictionaryWithDefault(v) => v.owner,
        obj::ObjectType::WipeoutVariables(v) => v.owner,
        obj::ObjectType::BlockVisibilityParameter(v) => v.owner,
        obj::ObjectType::DynamicBlock(v) => v.owner,
        obj::ObjectType::Associative(v) => v.owner,
        obj::ObjectType::ClassObject(v) => v.owner,
        obj::ObjectType::DataObject(v) => v.owner,
        obj::ObjectType::Field(v) => v.owner,
        obj::ObjectType::FieldList(v) => v.owner,
        obj::ObjectType::RegisteredClass(v) => v.owner,
        obj::ObjectType::DgnLineStyle(v) => v.owner,
        obj::ObjectType::ProxyObject(v) => v.owner,
        obj::ObjectType::Unknown { owner, .. } => *owner,
    }
}

fn handle_exists(doc: &CadDocument, target: Handle) -> bool {
    if target.is_null() || doc.objects.contains_key(&target) || doc.get_entity(target).is_some() {
        return true;
    }
    macro_rules! found { ($($field:ident),+) => { false $(|| doc.$field.iter().any(|entry| entry.handle() == target))+ }; }
    found!(
        layers,
        line_types,
        text_styles,
        block_records,
        dim_styles,
        app_ids,
        views,
        vports,
        ucss,
        vx_table
    )
}

fn insert_object(doc: &mut CadDocument, mut value: obj::ObjectType, owner: Handle) -> Handle {
    let handle = doc.allocate_handle();
    set_identity(&mut value, handle, owner);
    match &value {
        obj::ObjectType::BlockVisibilityParameter(value) => {
            doc.block_visibility_params.insert(handle, value.clone());
        }
        obj::ObjectType::ObjectContextData(value) => {
            doc.context_scales.insert(handle, value.scale);
        }
        obj::ObjectType::DynamicBlock(value) => {
            if let obj::DynamicBlockData::Representation(data) = &value.data {
                doc.block_representations.insert(handle, data.block);
            }
        }
        obj::ObjectType::Field(value) => {
            doc.fields.insert(
                handle,
                acadrust::document::FieldDef {
                    handle,
                    owner,
                    evaluator: value.evaluator_id.clone(),
                    code: value.code.clone(),
                    objects: value.referenced_objects.clone(),
                },
            );
        }
        _ => {}
    }
    doc.objects.insert(handle, value);
    handle
}

/// Rebuild the decoded side views that mirror selected database objects.
/// Native object mutation can change their payload without changing the
/// object handle, so these views must be refreshed after every edit.
pub(crate) fn sync_object_side_views(doc: &mut CadDocument, handle: Handle) {
    doc.block_visibility_params.remove(&handle);
    doc.context_scales.remove(&handle);
    doc.block_representations.remove(&handle);
    doc.fields.remove(&handle);
    doc.dgn_ls_definitions.remove(&handle);
    doc.dgn_ls_components.remove(&handle);
    let Some(value) = doc.objects.get(&handle) else {
        return;
    };
    match value {
        obj::ObjectType::BlockVisibilityParameter(value) => {
            doc.block_visibility_params.insert(handle, value.clone());
        }
        obj::ObjectType::ObjectContextData(value) => {
            doc.context_scales.insert(handle, value.scale);
        }
        obj::ObjectType::DynamicBlock(value) => {
            if let obj::DynamicBlockData::Representation(data) = &value.data {
                doc.block_representations.insert(handle, data.block);
            }
        }
        obj::ObjectType::Field(value) => {
            doc.fields.insert(
                handle,
                acadrust::document::FieldDef {
                    handle,
                    owner: value.owner,
                    evaluator: value.evaluator_id.clone(),
                    code: value.code.clone(),
                    objects: value.referenced_objects.clone(),
                },
            );
        }
        _ => {}
    }
}

#[no_mangle]
pub unsafe extern "C" fn ar_object_kind_count(out_count: *mut usize) -> Status {
    ffi_call(|| {
        *output(out_count)? = OBJECT_NAMES.len();
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_object_kind_name(index: usize, out_name: *mut StringView) -> Status {
    ffi_call(|| {
        *output(out_name)? = StringView::new(
            OBJECT_NAMES
                .get(index)
                .ok_or_else(|| not_found("object kind index is out of range"))?,
        );
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_object_kind_variant(index: usize, out_name: *mut StringView) -> Status {
    ffi_call(|| {
        let name = OBJECT_NAMES
            .get(index)
            .ok_or_else(|| not_found("object kind index is out of range"))?;
        *output(out_name)? =
            StringView::new(make_object(name)?.variant().expect("object enum variant"));
        Ok(())
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_object_add_default(
    doc: *mut Document,
    kind: *const c_char,
    owner: u64,
    out_handle: *mut u64,
) -> Status {
    ffi_call(|| {
        let out = output(out_handle)?;
        let value = make_object(input_str(kind)?)?;
        let doc = document_mut(doc)?;
        let owner = Handle::new(owner);
        if !handle_exists(doc, owner) {
            return Err(not_found("object owner was not found"));
        }
        *out = insert_object(doc, value, owner).value();
        Ok(())
    })
}

fn prepare_clone(value: &mut obj::ObjectType) -> FfiResult<()> {
    match value {
        obj::ObjectType::Unknown {
            raw_dwg_data,
            raw_dxf_codes,
            ..
        } if raw_dwg_data.is_some() || raw_dxf_codes.is_some() => {
            return Err(type_mismatch(
                "opaque object payload cannot be cloned under a new identity",
            ))
        }
        obj::ObjectType::RegisteredClass(v) if v.raw_dwg_data.is_some() => {
            return Err(type_mismatch(
                "opaque registered-class payload cannot be cloned under a new identity",
            ))
        }
        obj::ObjectType::ClassObject(v) if matches!(&v.data, obj::ClassObjectData::CsacDocumentOptions(data) if data.raw_dwg_data.is_some()) => {
            return Err(type_mismatch(
                "opaque class payload cannot be cloned under a new identity",
            ))
        }
        obj::ObjectType::XRecord(v) => {
            if !v.entries_complete && v.raw_dwg_data.is_some() {
                return Err(type_mismatch("incomplete opaque XRecord cannot be cloned"));
            }
            v.raw_dwg_data = None;
            v.raw_dwg_version = None;
        }
        obj::ObjectType::Layout(v) => v.raw_plot_settings_codes = None,
        obj::ObjectType::TableStyle(v) => v.raw_dxf_codes = None,
        obj::ObjectType::SortEntitiesTable(v) => {
            v.raw_dxf_codes = None;
            v.raw_dxf_version = None;
        }
        obj::ObjectType::Dictionary(v) => {
            v.hard_owner = false;
            v.hard_owner_entries.clear();
            v.reactors.clear();
            v.xdictionary_handle = None;
        }
        obj::ObjectType::DictionaryWithDefault(v) => v.hard_owner = false,
        obj::ObjectType::TableContent(v) => {
            v.common.raw_record = None;
            v.common.reactors.clear();
            v.common.xdictionary_handle = None;
        }
        _ => {}
    }
    Ok(())
}

#[no_mangle]
pub unsafe extern "C" fn ar_object_clone(
    doc: *mut Document,
    source: u64,
    owner: u64,
    out_handle: *mut u64,
) -> Status {
    ffi_call(|| {
        let out = output(out_handle)?;
        let doc = document_mut(doc)?;
        let source = Handle::new(source);
        let mut value = doc
            .objects
            .get(&source)
            .ok_or_else(|| not_found("source object was not found"))?
            .clone();
        let owner = if owner == 0 {
            object_owner(&value)
        } else {
            Handle::new(owner)
        };
        if !handle_exists(doc, owner) {
            return Err(not_found("destination owner was not found"));
        }
        prepare_clone(&mut value)?;
        clear_clone_links(&mut value);
        let xdata = doc.object_xdata.get(&source).cloned();
        let handle = insert_object(doc, value, owner);
        if let Some(xdata) = xdata {
            doc.object_xdata.insert(handle, xdata);
        }
        *out = handle.value();
        Ok(())
    })
}

unsafe fn borrowed_text<'a>(value: StringView) -> &'a str {
    std::str::from_utf8_unchecked(std::slice::from_raw_parts(value.data.cast(), value.len))
}

fn same_name(left: &str, right: &str) -> bool {
    acadrust::tables::normalize_name(left) == acadrust::tables::normalize_name(right)
}

fn references_handle(
    value: ValueView,
    target: Handle,
    excluded: *const c_void,
    dictionary_links: bool,
) -> bool {
    if value.data == excluded {
        return false;
    }
    unsafe {
        let mut name = StringView::new("");
        if ar_value_type_name(value, &mut name) != Status::Ok {
            return false;
        }
        let type_name = borrowed_text(name);
        if type_name == std::any::type_name::<Handle>() {
            let mut handle = 0;
            return ar_value_read_integer(value, &mut handle) == Status::Ok
                && handle as u64 == target.value();
        }
        let mut kind = 0;
        if ar_value_kind(value, &mut kind) != Status::Ok
            || !matches!(
                kind,
                AR_VALUE_STRUCT
                    | AR_VALUE_ENUM
                    | AR_VALUE_SEQUENCE
                    | AR_VALUE_OPTION
                    | AR_VALUE_MAP
            )
        {
            return false;
        }
        let mut len = 0;
        if ar_value_len(value, &mut len) != Status::Ok {
            return false;
        }
        if kind == AR_VALUE_MAP {
            len *= 2;
        }
        for index in 0..len {
            let mut child = ValueView::default();
            if ar_value_child(value, index, &mut name, &mut child) != Status::Ok {
                continue;
            }
            if dictionary_links
                && type_name == std::any::type_name::<obj::Dictionary>()
                && borrowed_text(name) == "entries"
            {
                continue;
            }
            if dictionary_links
                && type_name == std::any::type_name::<obj::DictionaryWithDefault>()
                && borrowed_text(name) == "entries"
            {
                continue;
            }
            if references_handle(child, target, excluded, dictionary_links) {
                return true;
            }
        }
        false
    }
}

fn is_referenced(
    doc: &CadDocument,
    target: Handle,
    excluded: *const c_void,
    dictionary_links: bool,
) -> bool {
    macro_rules! referenced { ($($field:ident),+) => { false $(|| references_handle(view(&doc.$field), target, excluded, dictionary_links))+ }; }
    referenced!(
        header,
        layers,
        line_types,
        text_styles,
        block_records,
        dim_styles,
        app_ids,
        views,
        vports,
        ucss,
        vx_table
    ) || doc
        .entities()
        .any(|value| references_handle(view(value), target, excluded, dictionary_links))
        || doc
            .objects
            .iter()
            .filter(|(handle, _)| **handle != target)
            .any(|(_, value)| references_handle(view(value), target, excluded, dictionary_links))
        || doc
            .object_xdata
            .iter()
            .filter(|(handle, _)| **handle != target)
            .any(|(_, value)| references_handle(view(value), target, excluded, dictionary_links))
        || doc
            .context_scales
            .iter()
            .any(|(handle, value)| *handle != target && *value == target)
        || doc
            .block_representations
            .iter()
            .any(|(handle, value)| *handle != target && *value == target)
        || doc
            .block_visibility_params
            .iter()
            .filter(|(handle, _)| **handle != target)
            .any(|(_, value)| references_handle(view(value), target, excluded, dictionary_links))
        || doc
            .fields
            .iter()
            .filter(|(handle, _)| **handle != target)
            .any(|(_, value)| references_handle(view(value), target, excluded, dictionary_links))
        || doc
            .dgn_ls_definitions
            .iter()
            .filter(|(handle, _)| **handle != target)
            .any(|(_, value)| references_handle(view(value), target, excluded, dictionary_links))
        || doc
            .dgn_ls_components
            .iter()
            .filter(|(handle, _)| **handle != target)
            .any(|(_, value)| references_handle(view(value), target, excluded, dictionary_links))
}

fn references_symbol(value: ValueView, table: i32, symbol: &str) -> bool {
    unsafe {
        let mut name = StringView::new("");
        let mut kind = 0;
        if ar_value_kind(value, &mut kind) != Status::Ok
            || !matches!(
                kind,
                AR_VALUE_STRUCT
                    | AR_VALUE_ENUM
                    | AR_VALUE_SEQUENCE
                    | AR_VALUE_OPTION
                    | AR_VALUE_MAP
            )
        {
            return false;
        }
        if ar_value_type_name(value, &mut name) != Status::Ok {
            return false;
        }
        let parent = borrowed_text(name).rsplit("::").next().unwrap_or("");
        let mut len = 0;
        if ar_value_len(value, &mut len) != Status::Ok {
            return false;
        }
        if kind == AR_VALUE_MAP {
            len *= 2;
        }
        for index in 0..len {
            let mut child = ValueView::default();
            if ar_value_child(value, index, &mut name, &mut child) != Status::Ok {
                continue;
            }
            let field = borrowed_text(name);
            let named_reference = match table {
                AR_TABLE_LAYER => {
                    field == "layer"
                        || (parent == "XDataValue" && {
                            let mut variant = StringView::new("");
                            ar_value_variant(value, &mut variant) == Status::Ok
                                && borrowed_text(variant) == "LayerName"
                        })
                }
                AR_TABLE_LINETYPE => matches!(
                    field,
                    "linetype" | "line_type" | "linetype_name" | "line_type_name"
                ),
                AR_TABLE_TEXT_STYLE => {
                    matches!(field, "text_style" | "text_style_name" | "dimtxsty")
                        || (field == "style" && matches!(parent, "Text" | "MText"))
                        || (field == "style_name"
                            && matches!(parent, "Shape" | "ArcAlignedTextData" | "RemoteTextData"))
                }
                AR_TABLE_DIM_STYLE => {
                    (parent == "DimensionBase" && field == "style_name")
                        || matches!(field, "dimension_style" | "dimension_style_name")
                }
                AR_TABLE_BLOCK_RECORD => {
                    matches!(
                        field,
                        "block_name"
                            | "dimblk_name"
                            | "dimblk1_name"
                            | "dimblk2_name"
                            | "dimldrblk_name"
                            | "dim_leader_arrow_block"
                            | "block_content_name"
                    )
                }
                AR_TABLE_APP_ID => parent == "ExtendedDataRecord" && field == "application_name",
                AR_TABLE_VIEW => field == "plot_view_name",
                AR_TABLE_UCS => field == "ucs_name",
                _ => false,
            };
            if named_reference {
                let mut child_kind = 0;
                if ar_value_kind(child, &mut child_kind) == Status::Ok
                    && child_kind == AR_VALUE_STRING
                {
                    let (mut data, mut count, mut stride) = (std::ptr::null(), 0, 0);
                    if ar_value_data(child, &mut data, &mut count, &mut stride) == Status::Ok
                        && same_name(
                            std::str::from_utf8_unchecked(std::slice::from_raw_parts(
                                data.cast(),
                                count,
                            )),
                            symbol,
                        )
                    {
                        return true;
                    }
                }
            }
            if references_symbol(child, table, symbol) {
                return true;
            }
        }
        false
    }
}

fn symbol_is_referenced(doc: &CadDocument, table: i32, name: &str) -> bool {
    // Header variables contain the named references that are not represented
    // by a handle (for example DIMBLK and the current UCS names).  They are
    // deliberately checked explicitly: their field names are drawing-header
    // names rather than the entity/table field names handled below.
    let header_reference = match table {
        AR_TABLE_LAYER => same_name(&doc.header.current_layer_name, name),
        AR_TABLE_LINETYPE => same_name(&doc.header.current_linetype_name, name),
        AR_TABLE_TEXT_STYLE => same_name(&doc.header.current_text_style_name, name),
        AR_TABLE_DIM_STYLE => same_name(&doc.header.current_dimstyle_name, name),
        AR_TABLE_BLOCK_RECORD => [
            &doc.header.dim_arrow_block,
            &doc.header.dim_arrow_block1,
            &doc.header.dim_arrow_block2,
            &doc.header.dim_leader_arrow_block,
        ]
        .iter()
        .any(|value| !value.is_empty() && same_name(value, name)),
        AR_TABLE_UCS => [
            &doc.header.ucs_base,
            &doc.header.model_space_ucs_name,
            &doc.header.paper_space_ucs_name,
        ]
        .iter()
        .any(|value| !value.is_empty() && same_name(value, name)),
        _ => false,
    };

    header_reference
        || doc
            .entities()
            .any(|value| references_symbol(view(value), table, name))
        || doc
            .objects
            .values()
            .any(|value| references_symbol(view(value), table, name))
        // Table records can contain named references to records in another
        // table (a layer's linetype is the common example).  Walk every
        // modeled table so those references participate in deletion checks.
        || doc
            .layers
            .iter()
            .any(|value| references_symbol(view(value), table, name))
        || doc
            .line_types
            .iter()
            .any(|value| references_symbol(view(value), table, name))
        || doc
            .text_styles
            .iter()
            .any(|value| references_symbol(view(value), table, name))
        || doc
            .block_records
            .iter()
            .any(|value| references_symbol(view(value), table, name))
        || doc
            .dim_styles
            .iter()
            .any(|value| references_symbol(view(value), table, name))
        || doc
            .app_ids
            .iter()
            .any(|value| references_symbol(view(value), table, name))
        || doc
            .views
            .iter()
            .any(|value| references_symbol(view(value), table, name))
        || doc
            .vports
            .iter()
            .any(|value| references_symbol(view(value), table, name))
        || doc
            .ucss
            .iter()
            .any(|value| references_symbol(view(value), table, name))
        || doc
            .vx_table
            .iter()
            .any(|value| references_symbol(view(value), table, name))
        || doc
            .object_xdata
            .values()
            .any(|value| references_symbol(view(value), table, name))
}

/// Return an owner whose extension dictionary is `target`.
///
/// `CadDocument` keeps extension dictionaries for non-entity owners in a
/// private side map.  `is_referenced` intentionally ignores dictionary entry
/// links because object removal cleans those links, so that side map must be
/// checked through the public resolver before deleting a dictionary object.
fn extension_dictionary_owner(doc: &CadDocument, target: Handle) -> Option<Handle> {
    if let Some(owner) = doc.entities().find_map(|value| {
        let owner = value.common().handle;
        (doc.extension_dictionary_handle(owner) == Some(target)).then_some(owner)
    }) {
        return Some(owner);
    }

    if let Some(owner) = doc
        .objects
        .keys()
        .copied()
        .find(|owner| doc.extension_dictionary_handle(*owner) == Some(target))
    {
        return Some(owner);
    }

    macro_rules! table_owner {
        ($field:ident) => {
            if let Some(owner) = doc.$field.iter().find_map(|value| {
                let owner = value.handle();
                (doc.extension_dictionary_handle(owner) == Some(target)).then_some(owner)
            }) {
                return Some(owner);
            }
        };
    }
    table_owner!(layers);
    table_owner!(line_types);
    table_owner!(text_styles);
    table_owner!(block_records);
    table_owner!(dim_styles);
    table_owner!(app_ids);
    table_owner!(views);
    table_owner!(vports);
    table_owner!(ucss);
    table_owner!(vx_table);
    None
}

#[no_mangle]
pub unsafe extern "C" fn ar_object_remove(doc: *mut Document, handle: u64) -> Status {
    ffi_call(|| {
        let doc = document_mut(doc)?;
        let handle = Handle::new(handle);
        let source = doc
            .objects
            .get(&handle)
            .ok_or_else(|| not_found("object was not found"))?;
        if extension_dictionary_owner(doc, handle).is_some() {
            return Err(invalid(
                "extension dictionary is still attached to another database record",
            ));
        }
        if is_referenced(doc, handle, view(source).data, true) {
            return Err(invalid(
                "object is still referenced by another database record",
            ));
        }
        for value in doc.objects.values_mut() {
            match value {
                obj::ObjectType::Dictionary(value) => {
                    value.entries.retain(|(_, target)| *target != handle);
                    value.hard_owner_entries.retain(|key| {
                        value
                            .entries
                            .iter()
                            .any(|(name, _)| name.eq_ignore_ascii_case(key))
                    });
                }
                obj::ObjectType::DictionaryWithDefault(value) => {
                    value.entries.retain(|(_, target)| *target != handle)
                }
                _ => {}
            }
        }
        doc.objects.remove(&handle);
        doc.block_visibility_params.remove(&handle);
        doc.context_scales.remove(&handle);
        doc.block_representations.remove(&handle);
        doc.fields.remove(&handle);
        doc.dgn_ls_definitions.remove(&handle);
        doc.dgn_ls_components.remove(&handle);
        doc.object_xdata.remove(&handle);
        Ok(())
    })
}

enum EntrySelection<'a> {
    Name(&'a str),
    Index(usize),
}

fn select_table_entry(
    doc: &CadDocument,
    kind: i32,
    selection: EntrySelection<'_>,
) -> FfiResult<(String, Handle)> {
    macro_rules! select {
        ($field:ident) => {{
            let entry = match selection {
                EntrySelection::Name(name) => doc.$field.get(name),
                EntrySelection::Index(index) => doc.$field.iter().nth(index),
            }
            .ok_or_else(|| not_found("table entry was not found"))?;
            Ok((entry.name().to_owned(), entry.handle()))
        }};
    }
    match kind {
        AR_TABLE_LAYER => select!(layers),
        AR_TABLE_LINETYPE => select!(line_types),
        AR_TABLE_TEXT_STYLE => select!(text_styles),
        AR_TABLE_BLOCK_RECORD => select!(block_records),
        AR_TABLE_DIM_STYLE => select!(dim_styles),
        AR_TABLE_APP_ID => select!(app_ids),
        AR_TABLE_VIEW => select!(views),
        AR_TABLE_VPORT => select!(vports),
        AR_TABLE_UCS => select!(ucss),
        AR_TABLE_VX => select!(vx_table),
        _ => Err(invalid("unknown symbol table kind")),
    }
}

fn remove_table_entry(
    doc: &mut CadDocument,
    kind: i32,
    name: &str,
    handle: Handle,
) -> FfiResult<()> {
    macro_rules! remove {
        ($field:ident) => {{
            let source = doc
                .$field
                .iter()
                .find(|entry| entry.handle() == handle)
                .ok_or_else(|| not_found("table entry was not found"))?;
            let handle = source.handle();
            if is_referenced(doc, handle, view(source).data, false)
                || symbol_is_referenced(doc, kind, name)
            {
                return Err(invalid("table entry is still referenced"));
            }
            let entries: Vec<_> = doc
                .$field
                .iter()
                .filter(|entry| entry.handle() != handle)
                .cloned()
                .collect();
            doc.$field.clear();
            for entry in entries {
                doc.$field.add_allow_duplicate(entry);
            }
            doc.object_xdata.remove(&handle);
            if kind == AR_TABLE_VX {
                doc.vx_control_entries.retain(|entry| *entry != handle);
            }
        }};
    }
    match kind {
        AR_TABLE_LAYER => {
            if name.eq_ignore_ascii_case("0")
                || same_name(&doc.header.current_layer_name, name)
                || doc.entities().any(|e| same_name(&e.common().layer, name))
            {
                return Err(invalid("layer is a default, current, or used layer"));
            }
            remove!(layers);
        }
        AR_TABLE_LINETYPE => {
            if ["BYLAYER", "BYBLOCK", "CONTINUOUS"]
                .iter()
                .any(|s| name.eq_ignore_ascii_case(s))
                || same_name(&doc.header.current_linetype_name, name)
                || doc
                    .entities()
                    .any(|e| same_name(&e.common().linetype, name))
                || doc.layers.iter().any(|e| same_name(&e.line_type, name))
            {
                return Err(invalid("linetype is a default, current, or used linetype"));
            }
            remove!(line_types);
        }
        AR_TABLE_TEXT_STYLE => {
            if name.eq_ignore_ascii_case("STANDARD")
                || same_name(&doc.header.current_text_style_name, name)
            {
                return Err(invalid("cannot remove the default or current text style"));
            }
            remove!(text_styles);
        }
        AR_TABLE_BLOCK_RECORD => {
            let block = doc
                .block_records
                .get(name)
                .ok_or_else(|| not_found("block record was not found"))?;
            if !block.entity_handles.is_empty()
                || !block.layout.is_null()
                || name.to_ascii_uppercase().starts_with("*MODEL_SPACE")
                || name.to_ascii_uppercase().starts_with("*PAPER_SPACE")
                || doc
                    .entities()
                    .any(|e| matches!(e, EntityType::Insert(v) if same_name(&v.block_name, name)))
            {
                return Err(invalid(
                    "block record is populated, referenced, or belongs to a layout",
                ));
            }
            let markers = [block.block_entity_handle, block.block_end_handle];
            remove!(block_records);
            for marker in markers {
                doc.remove_entity(marker);
            }
        }
        AR_TABLE_DIM_STYLE => {
            if name.eq_ignore_ascii_case("STANDARD")
                || same_name(&doc.header.current_dimstyle_name, name)
            {
                return Err(invalid(
                    "cannot remove the default or current dimension style",
                ));
            }
            remove!(dim_styles);
        }
        AR_TABLE_APP_ID => {
            if name.eq_ignore_ascii_case("ACAD") {
                return Err(invalid("cannot remove ACAD registration"));
            }
            remove!(app_ids);
        }
        AR_TABLE_VIEW => remove!(views),
        AR_TABLE_VPORT => {
            if name.eq_ignore_ascii_case("*ACTIVE") {
                return Err(invalid("cannot remove the active viewport"));
            }
            remove!(vports);
        }
        AR_TABLE_UCS => remove!(ucss),
        AR_TABLE_VX => remove!(vx_table),
        _ => return Err(invalid("unknown symbol table kind")),
    }

    Ok(())
}

#[no_mangle]
pub unsafe extern "C" fn ar_table_remove(
    doc: *mut Document,
    kind: i32,
    name: *const c_char,
) -> Status {
    ffi_call(|| {
        let name = input_str(name)?;
        let doc = document_mut(doc)?;
        let (name, handle) = select_table_entry(doc, kind, EntrySelection::Name(name))?;
        remove_table_entry(doc, kind, &name, handle)
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_table_remove_at(doc: *mut Document, kind: i32, index: usize) -> Status {
    ffi_call(|| {
        let doc = document_mut(doc)?;
        let (name, handle) = select_table_entry(doc, kind, EntrySelection::Index(index))?;
        remove_table_entry(doc, kind, &name, handle)
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_class_add(
    doc: *mut Document,
    dxf_name: *const c_char,
    cpp_name: *const c_char,
    application: *const c_char,
    is_entity: u8,
    out_index: *mut usize,
) -> Status {
    ffi_call(|| {
        let out = output(out_index)?;
        let (name, cpp, application) = (
            input_str(dxf_name)?,
            input_str(cpp_name)?,
            input_str(application)?,
        );
        if name.is_empty() || cpp.is_empty() || is_entity > 1 {
            return Err(invalid(
                "class names must be nonempty and is_entity must be 0 or 1",
            ));
        }
        let doc = document_mut(doc)?;
        if doc.classes.contains(name) {
            return Err(invalid("class name already exists"));
        }
        if doc.classes.len() >= i16::MAX as usize - 500 {
            return Err(invalid("class table is full"));
        }
        let mut class = if is_entity == 1 {
            acadrust::classes::DxfClass::new_entity(name, cpp)
        } else {
            acadrust::classes::DxfClass::new(name, cpp)
        };
        class.application_name = application.into();
        *out = doc.classes.len();
        doc.classes.add_or_update(class);
        Ok(())
    })
}

fn object_class(value: &obj::ObjectType) -> &str {
    match value {
        obj::ObjectType::DynamicBlock(v) => &v.dxf_name,
        obj::ObjectType::Associative(v) => &v.dxf_name,
        obj::ObjectType::ClassObject(v) => v.dxf_name(),
        obj::ObjectType::DataObject(v) => v.dxf_name(),
        obj::ObjectType::ObjectContextData(v) => v.class_name(),
        obj::ObjectType::DgnLineStyle(v) => v.dxf_name(),
        obj::ObjectType::RegisteredClass(v) => &v.dxf_name,
        obj::ObjectType::Unknown { type_name, .. } => type_name,
        obj::ObjectType::ImageDefinition(_) => "IMAGEDEF",
        obj::ObjectType::ImageDefinitionReactor(_) => "IMAGEDEF_REACTOR",
        obj::ObjectType::MultiLeaderStyle(_) => "MLEADERSTYLE",
        obj::ObjectType::DictionaryVariable(_) => "DICTIONARYVAR",
        obj::ObjectType::DictionaryWithDefault(_) => "ACDBDICTIONARYWDFLT",
        obj::ObjectType::PlaceHolder(_) => "ACDBPLACEHOLDER",
        obj::ObjectType::BookColor(_) => "DBCOLOR",
        obj::ObjectType::SortEntitiesTable(_) => "SORTENTSTABLE",
        obj::ObjectType::ProxyObject(_) => "ACAD_PROXY_OBJECT",
        obj::ObjectType::UnderlayDefinition(v) => match v.underlay_type {
            acadrust::entities::UnderlayType::Pdf => "PDFDEFINITION",
            acadrust::entities::UnderlayType::Dwf => "DWFDEFINITION",
            acadrust::entities::UnderlayType::Dgn => "DGNDEFINITION",
        },
        _ => value.variant().unwrap_or(""),
    }
}

#[no_mangle]
pub unsafe extern "C" fn ar_class_remove(doc: *mut Document, index: usize) -> Status {
    ffi_call(|| {
        let doc = document_mut(doc)?;
        let class = doc
            .classes
            .iter()
            .nth(index)
            .ok_or_else(|| not_found("class index was not found"))?;
        let name = &class.dxf_name;
        let class_number = i32::from(class.class_number);
        if doc.objects.values().any(|v| matches!(v, obj::ObjectType::ProxyObject(v)
            if v.class_id == class_number || v.class_id == 500 + index as i32 || same_name(&v.dxf_subclass, name)))
            || doc.entities().any(|v| matches!(v, EntityType::Extended(v)
                if matches!(&v.data, acadrust::entities::ExtendedEntityData::Proxy(v)
                    if v.class_id == class_number || v.class_id == 500 + index as i32 || same_name(&v.dxf_subclass, name)))) {
            return Err(invalid("class is still referenced by a proxy instance"));
        }
        if doc
            .objects
            .values()
            .any(|v| same_name(object_class(v), name))
            || doc.entities().any(|v| match v {
                EntityType::Extended(v) => same_name(v.class_name(), name),
                EntityType::Unknown(v) => same_name(&v.dxf_name, name),
                _ => same_name(v.as_entity().entity_type(), name),
            })
        {
            return Err(invalid("class still has instances"));
        }
        if index + 1 < doc.classes.len()
            && (doc
                .entities()
                .any(|e| e.common().raw_record.is_some() || matches!(e, EntityType::Unknown(_)))
                || doc.objects.values().any(|v| {
                    matches!(
                        v,
                        obj::ObjectType::Unknown { .. }
                            | obj::ObjectType::ProxyObject(_)
                            | obj::ObjectType::RegisteredClass(_)
                    )
                }))
        {
            return Err(invalid(
                "removing an interior class would shift opaque positional class references",
            ));
        }
        let mut classes: Vec<_> = doc.classes.iter().cloned().collect();
        classes.remove(index);
        doc.classes.clear();
        for (index, mut class) in classes.into_iter().enumerate() {
            class.class_number = (500 + index) as i16;
            doc.classes.push_preserving(class);
        }
        Ok(())
    })
}

pub(crate) fn move_entity(doc: &mut CadDocument, handle: Handle, block: Handle) -> FfiResult<()> {
    let source = doc
        .get_entity(handle)
        .ok_or_else(|| not_found("entity was not found"))?;
    if matches!(
        source,
        EntityType::Block(_)
            | EntityType::BlockEnd(_)
            | EntityType::Seqend(_)
            | EntityType::AttributeEntity(_)
    ) {
        return Err(type_mismatch(
            "structural and attribute subentities cannot be moved independently",
        ));
    }
    let is_viewport = matches!(source, EntityType::Viewport(_));
    let target = doc
        .block_records
        .iter()
        .find(|b| b.handle == block)
        .ok_or_else(|| not_found("destination block record was not found"))?;
    let layout_handle = target.layout;
    let entity_mode = if block == doc.header.model_space_block_handle {
        2
    } else if !layout_handle.is_null()
        || target.name.to_ascii_uppercase().starts_with("*PAPER_SPACE")
    {
        1
    } else {
        0
    };
    for record in doc.block_records.iter_mut() {
        record.entity_handles.retain(|entry| *entry != handle);
    }
    doc.block_records
        .iter_mut()
        .find(|b| b.handle == block)
        .expect("validated block")
        .entity_handles
        .push(handle);
    let value = doc.get_entity_mut(handle).expect("validated entity");
    value.common_mut().owner_handle = block;
    value.common_mut().entity_mode = Some(entity_mode);
    if is_viewport {
        for value in doc.objects.values_mut() {
            if let obj::ObjectType::Layout(layout) = value {
                layout.viewports.retain(|entry| *entry != handle);
                if layout.viewport == handle {
                    layout.viewport = layout.viewports.first().copied().unwrap_or(Handle::NULL);
                }
                if layout.block_record == block {
                    layout.viewports.push(handle);
                    if layout.viewport.is_null() {
                        layout.viewport = handle;
                    }
                }
            }
        }
    }
    Ok(())
}

#[no_mangle]
pub unsafe extern "C" fn ar_entity_move_to_block_name(
    doc: *mut Document,
    handle: u64,
    block: *const c_char,
) -> Status {
    ffi_call(|| {
        let name = input_str(block)?;
        let doc = document_mut(doc)?;
        let block = doc
            .block_records
            .get(name)
            .ok_or_else(|| not_found("destination block record was not found"))?
            .handle;
        move_entity(doc, Handle::new(handle), block)
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_entity_move_to_layout(
    doc: *mut Document,
    handle: u64,
    layout: *const c_char,
) -> Status {
    ffi_call(|| {
        let name = input_str(layout)?;
        let doc = document_mut(doc)?;
        let block = doc
            .objects
            .values()
            .find_map(|v| match v {
                obj::ObjectType::Layout(v) if same_name(&v.name, name) => Some(v.block_record),
                _ => None,
            })
            .ok_or_else(|| not_found("destination layout was not found"))?;
        move_entity(doc, Handle::new(handle), block)
    })
}

#[no_mangle]
pub unsafe extern "C" fn ar_entity_clone(
    doc: *mut Document,
    source: u64,
    out_handle: *mut u64,
) -> Status {
    ffi_call(|| {
        let out = output(out_handle)?;
        let doc = document_mut(doc)?;
        let source = doc
            .get_entity(Handle::new(source))
            .ok_or_else(|| not_found("source entity was not found"))?;
        if matches!(source, EntityType::Unknown(v) if v.raw_dwg_data.is_some() || v.raw_dxf_codes.is_some())
        {
            return Err(type_mismatch(
                "opaque entity payload cannot be cloned under a new identity",
            ));
        }
        if matches!(source, EntityType::Extended(v) if match &v.data {
            acadrust::entities::ExtendedEntityData::Format(v) => v.raw_dwg_data.is_some() || v.raw_dxf_codes.is_some(),
            acadrust::entities::ExtendedEntityData::LayoutPrintConfig(v) => v.raw_dwg_data.is_some(),
            _ => false,
        }) {
            return Err(type_mismatch(
                "opaque extended entity payload cannot be cloned under a new identity",
            ));
        }
        if matches!(
            source,
            EntityType::Block(_)
                | EntityType::BlockEnd(_)
                | EntityType::Seqend(_)
                | EntityType::AttributeEntity(_)
        ) {
            return Err(type_mismatch(
                "structural and attribute subentities cannot be cloned independently",
            ));
        }
        let owner = source.common().owner_handle;
        let mode = source.common().entity_mode;
        let is_viewport = matches!(source, EntityType::Viewport(_));
        let mut value = source.clone();
        crate::entities::clear_identity(&mut value);
        value.common_mut().owner_handle = owner;
        value.common_mut().entity_mode = mode;
        let handle = doc.add_entity(value)?;
        if is_viewport {
            for object in doc.objects.values_mut() {
                if let obj::ObjectType::Layout(layout) = object {
                    if layout.block_record == owner {
                        layout.viewports.push(handle);
                        if layout.viewport.is_null() {
                            layout.viewport = handle;
                        }
                    }
                }
            }
        }
        *out = handle.value();
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn drawing() -> Document {
        Document::new(CadDocument::new())
    }

    #[test]
    fn object_removal_cleans_dictionary_links_and_side_indexes() {
        let mut doc = drawing();
        let owner = insert_object(
            &mut doc.inner,
            make_object("DICTIONARY").unwrap(),
            Handle::NULL,
        );
        let child = insert_object(
            &mut doc.inner,
            make_object("CONTEXT_ANNOTSCALE").unwrap(),
            owner,
        );
        let obj::ObjectType::Dictionary(dictionary) = doc.inner.objects.get_mut(&owner).unwrap()
        else {
            unreachable!()
        };
        dictionary.add_entry("CHILD", child);
        dictionary.set_entry_hard_owner("CHILD", true);
        assert!(doc.inner.context_scales.contains_key(&child));
        unsafe {
            assert_eq!(ar_object_remove(&mut doc, child.value()), Status::Ok);
        }
        assert!(!doc.inner.context_scales.contains_key(&child));
        let obj::ObjectType::Dictionary(dictionary) = &doc.inner.objects[&owner] else {
            unreachable!()
        };
        assert!(dictionary.entries.is_empty());
        assert!(dictionary.hard_owner_entries.is_empty());
    }

    #[test]
    fn native_clone_keeps_logical_xdata_and_detaches_extension_ownership() {
        let mut doc = drawing();
        let source = insert_object(
            &mut doc.inner,
            make_object("DICTIONARY").unwrap(),
            Handle::NULL,
        );
        let obj::ObjectType::Dictionary(dictionary) = doc.inner.objects.get_mut(&source).unwrap()
        else {
            unreachable!()
        };
        dictionary.reactors.push(Handle::new(123));
        dictionary.xdictionary_handle = Some(Handle::new(124));
        let mut xdata = acadrust::xdata::ExtendedData::new();
        xdata.add_record(acadrust::xdata::ExtendedDataRecord::new("APP"));
        doc.inner.object_xdata.insert(source, xdata.clone());
        let mut cloned = 0;
        unsafe {
            assert_eq!(
                ar_object_clone(&mut doc, source.value(), 0, &mut cloned),
                Status::Ok
            );
        }
        let cloned = Handle::new(cloned);
        let obj::ObjectType::Dictionary(dictionary) = &doc.inner.objects[&cloned] else {
            unreachable!()
        };
        assert_eq!(dictionary.handle, cloned);
        assert!(dictionary.reactors.is_empty());
        assert_eq!(dictionary.xdictionary_handle, None);
        assert_eq!(doc.inner.object_xdata[&cloned], xdata);
    }

    #[test]
    fn symbol_removal_checks_string_references_without_cached_handles() {
        let mut doc = drawing();
        let handle = doc.inner.allocate_handle();
        let mut style = acadrust::tables::TextStyle::new("UsedStyle");
        style.handle = handle;
        doc.inner.text_styles.add(style).unwrap();
        let text = acadrust::entities::Text {
            style: "UsedStyle".into(),
            ..Default::default()
        };
        doc.inner.add_entity(EntityType::Text(text)).unwrap();
        unsafe {
            assert_eq!(
                ar_table_remove(&mut doc, AR_TABLE_TEXT_STYLE, c"UsedStyle".as_ptr()),
                Status::InvalidArgument
            );
        }
        assert!(doc.inner.text_styles.contains("UsedStyle"));

        let mut unicode_style = acadrust::tables::TextStyle::new("F\u{0131}eld");
        unicode_style.handle = doc.inner.allocate_handle();
        doc.inner.text_styles.add(unicode_style).unwrap();
        let text = acadrust::entities::Text {
            style: "FIELD".into(),
            ..Default::default()
        };
        doc.inner.add_entity(EntityType::Text(text)).unwrap();
        unsafe {
            assert_eq!(
                ar_table_remove(&mut doc, AR_TABLE_TEXT_STYLE, c"FIELD".as_ptr()),
                Status::InvalidArgument
            );
        }
        assert!(doc.inner.text_styles.contains("FIELD"));
    }

    #[test]
    fn opaque_clones_and_positional_class_removal_fail_before_mutation() {
        let mut doc = drawing();
        let mut unknown = make_object("UNKNOWN").unwrap();
        if let obj::ObjectType::Unknown { raw_dwg_data, .. } = &mut unknown {
            *raw_dwg_data = Some(vec![1, 2, 3]);
        }
        let source = insert_object(&mut doc.inner, unknown, Handle::NULL);
        let before = doc.inner.objects.len();
        let mut cloned = 0;
        unsafe {
            assert_eq!(
                ar_object_clone(&mut doc, source.value(), 0, &mut cloned),
                Status::TypeMismatch
            );
        }
        assert_eq!(doc.inner.objects.len(), before);
        let first = doc.inner.classes.len();
        doc.inner
            .classes
            .add_or_update(acadrust::classes::DxfClass::new("CUSTOM1", "Cpp1"));
        doc.inner
            .classes
            .add_or_update(acadrust::classes::DxfClass::new("CUSTOM2", "Cpp2"));
        unsafe {
            assert_eq!(ar_class_remove(&mut doc, first), Status::InvalidArgument);
        }
        assert!(doc.inner.classes.contains("CUSTOM1"));
        unsafe {
            assert_eq!(ar_class_remove(&mut doc, first + 1), Status::Ok);
        }
    }

    #[test]
    fn subtype_constructors_keep_native_class_metadata_in_sync() {
        let obj::ObjectType::Associative(surface) =
            make_object("ASSOCIATIVE_SURFACE_EDGECHAMFER").unwrap()
        else {
            unreachable!()
        };
        assert_eq!(surface.dxf_name, "ASSOCEDGECHAMFERACTIONBODY");
        assert_eq!(surface.cpp_class_name, "AcDbAssocEdgeChamferActionBody");
        let obj::ObjectType::Associative(array) = make_object("ASSOCARRAYPATHPARAMETERS").unwrap()
        else {
            unreachable!()
        };
        assert_eq!(array.dxf_name, "ASSOCARRAYPATHPARAMETERS");
        assert_eq!(array.cpp_class_name, "AcDbAssocArrayPathParameters");
        let obj::ObjectType::ObjectContextData(context) =
            make_object("CONTEXT_DIM_ANGULAR").unwrap()
        else {
            unreachable!()
        };
        assert_eq!(context.class_name(), "ACDB_ANGDIMOBJECTCONTEXTDATA_CLASS");
        let obj::ObjectType::DynamicBlock(block) =
            make_object("DYNAMICBLOCK_LINEARPARAMETER").unwrap()
        else {
            unreachable!()
        };
        assert_eq!(block.dxf_name, "BLOCKLINEARPARAMETER");
        assert_eq!(block.cpp_class_name, "AcDbBlockLinearParameter");
    }

    #[test]
    fn moving_viewports_updates_both_layout_lists_and_native_owner() {
        let mut doc = drawing();
        doc.inner.add_layout("Second").unwrap();
        let first_layout = doc
            .inner
            .objects
            .values()
            .find_map(|v| match v {
                obj::ObjectType::Layout(v) if v.name == "Layout1" => {
                    Some((v.handle, v.block_record))
                }
                _ => None,
            })
            .unwrap();
        let second_layout = doc
            .inner
            .objects
            .values()
            .find_map(|v| match v {
                obj::ObjectType::Layout(v) if v.name == "Second" => {
                    Some((v.handle, v.block_record))
                }
                _ => None,
            })
            .unwrap();
        let viewport = doc
            .inner
            .add_entity_to_layout(EntityType::Viewport(Box::default()), "Layout1")
            .unwrap();
        move_entity(&mut doc.inner, viewport, second_layout.1).unwrap();
        assert_eq!(
            doc.inner
                .get_entity(viewport)
                .unwrap()
                .common()
                .owner_handle,
            second_layout.1
        );
        for (id, contains) in [(first_layout.0, false), (second_layout.0, true)] {
            let obj::ObjectType::Layout(layout) = &doc.inner.objects[&id] else {
                unreachable!()
            };
            assert_eq!(layout.viewports.contains(&viewport), contains);
        }
    }
}
