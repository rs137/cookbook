pub struct CommandNames<'a> {
    pub module_command_name: &'a str,
    pub module_command_name_short: &'a str,
    pub module_command_name_long: &'a str,
    pub list_command_name: &'a str,
}

impl<'a> CommandNames<'a> {
    pub fn create() -> CommandNames<'a> {
        CommandNames {
            module_command_name: "module",
            module_command_name_short: "m",
            module_command_name_long: "module",
            list_command_name: "list"
        }
    }
}