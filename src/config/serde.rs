/// 序列化时的配置信息
pub trait SerdeConfig {
    /// 无论如何，字段数目都固定, 不需要的字段使用null填充 true
    /// 只提供需要的字段,其他缺省
    fn full_field(&self) -> bool {
        true
    }

    /// 标记基本响应状态
    /// - true 正常响应
    /// - false 异常响应
    ///
    /// is-ok
    ///
    /// Some() 标记，字段为提供的名称
    /// None 不标记
    fn signed_base_status(&self) -> bool {
        true
    }
    /// 异常码 位置标记
    ///
    /// extra-code
    ///
    /// Some() 添加异常码标记
    /// None 不添加异常码标记
    #[cfg(feature = "extra-code")]
    fn extra_code_local(&self) -> bool {
        true
    }
}

#[derive(serde::Deserialize)]
pub enum BodyType {
    #[serde(rename = "json")]
    Json,
}
