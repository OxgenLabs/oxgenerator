use crate::core::error::OxgenError;
use crate::core::generator::Generator;
use crate::core::generator_context::GeneratorContext;
use crate::core::naming::Name;
use crate::core::result::OxgenResult;
use crate::generators::{
    controller::ControllerGenerator, dto::DtoGenerator, model::ModelGenerator,
    route::RouteGenerator, service::ServiceGenerator,
};

pub struct ModuleGenerator {
    name: Name,
    context: GeneratorContext,
    collection: Option<String>,
}

impl ModuleGenerator {
    pub fn new(name: Name, context: GeneratorContext, collection: Option<String>) -> Self {
        Self {
            name,
            context,
            collection,
        }
    }

    fn validate_collection(&self) -> OxgenResult<()> {
        if self.collection.is_some() && !self.context.database.supports_collection() {
            return Err(OxgenError::CollectionRequiresMongoDb);
        }

        Ok(())
    }

    fn generate_model(&self) -> OxgenResult<()> {
        ModelGenerator::new(self.name.clone(), self.context).generate()
    }

    fn generate_dto(&self) -> OxgenResult<()> {
        DtoGenerator::new(self.name.clone(), self.context).generate()
    }

    fn generate_service(&self) -> OxgenResult<()> {
        ServiceGenerator::new(self.name.clone(), self.context).generate()
    }

    fn generate_controller(&self) -> OxgenResult<()> {
        ControllerGenerator::new(self.name.clone(), self.context, self.collection.clone())
            .generate()
    }

    fn generate_route(&self) -> OxgenResult<()> {
        RouteGenerator::new(self.name.clone(), self.context).generate()
    }
}

impl Generator for ModuleGenerator {
    fn generate(&self) -> OxgenResult<()> {
        self.validate_collection()?;

        self.generate_model()?;
        self.generate_dto()?;
        self.generate_service()?;
        self.generate_controller()?;
        self.generate_route()?;

        Ok(())
    }
}
