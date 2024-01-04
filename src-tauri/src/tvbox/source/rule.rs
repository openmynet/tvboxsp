#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    // 针对多个主机
    pub hosts: Option<Vec<String>>,
    pub name: Option<String>,
    pub regex: Option<Vec<String>>,
    // 只针对1格主机
    pub host: Option<String>,
    pub rule: Option<Vec<String>>,
}
