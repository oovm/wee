use super::*;


impl Store {
    pub fn check_cargo(&mut self) -> Result<()> {
        let toml: Toml = fs::read_to_string("Cargo.toml")?.parse()?;
        let metadata = toml.get("package")?.as_table()?.get("metadata")?.as_table()?;
        self.check_env(metadata).ok();
        self.check_wee(metadata).ok();
        Ok(())
    }
    fn check_env(&mut self, metadata: &Table) -> Result<()> {
        let env = metadata.get("env")?.as_table()?;
        for (k, v) in env {
            match v {
                Toml::String(str) => {
                    self.secrets.insert(Box::from(k.clone()), str.clone());
                }
                _ => println!("❌ `{}` is not a valid env variable", k),
            }
        }
        Ok(())
    }
    fn check_wee(&mut self, metadata: &Table) -> Result<()> {
        let wee = metadata.get("wee")?.as_table()?;
        for (k, v) in wee {
            match v {
                Toml::String(str) => {
                    self.scripts.insert(Box::from(k.clone()), str.clone());
                }
                _ => println!("❌ `{}` is not a valid script", k),
            }
        }
        Ok(())
    }
}