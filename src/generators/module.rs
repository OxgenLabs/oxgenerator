use crate::core::generator::Generator;
use crate::core::naming::Name;
use crate::core::result::OxgenResult;
use crate::generators::{
    controller::ControllerGenerator, dto::DtoGenerator, model::ModelGenerator,
    route::RouteGenerator, service::ServiceGenerator,
};

pub struct ModuleGenerator {
    name: Name,
    force: bool,
    dry_run: bool,
    database: String,
    collection: Option<String>
}

impl ModuleGenerator {
    pub fn new(name: Name, force: bool, dry_run: bool, database: String, collection: Option<String>) -> Self {
        Self {
            name,
            force,
            dry_run,
            database,
            collection
        }
    }

    fn generate_model(&self) -> OxgenResult<()> {
        ModelGenerator::new(
            self.name.clone(),
            self.force,
            self.dry_run,
            self.database.clone()
        )
        .generate()
    }

    fn generate_dto(&self) -> OxgenResult<()> {
        DtoGenerator::new(
            self.name.clone(),
            self.force,
            self.dry_run,
            self.database.clone()
        )
        .generate()
    }

    fn generate_service(&self) -> OxgenResult<()> {
        ServiceGenerator::new(
            self.name.clone(),
            self.force,
            self.dry_run,
            self.database.clone()
        )
        .generate()
    }

    fn generate_controller(&self) -> OxgenResult<()> {
        ControllerGenerator::new(
            self.name.clone(),
            self.force,
            self.dry_run,
            self.database.clone(),
            self.collection.clone()
        )
        .generate()
    }

    fn generate_route(&self) -> OxgenResult<()> {
        RouteGenerator::new(
            self.name.clone(),
            self.force,
            self.dry_run,
            self.database.clone()
        )
        .generate()
    }
}

impl Generator for ModuleGenerator {
    fn generate(&self) -> OxgenResult<()> {
        self.generate_model()?;
        self.generate_dto()?;
        self.generate_service()?;
        self.generate_controller()?;
        self.generate_route()?;

        Ok(())
    }
}
