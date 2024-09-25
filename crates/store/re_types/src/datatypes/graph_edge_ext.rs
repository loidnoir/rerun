use super::GraphNodeId;

impl<T: Into<GraphNodeId>> From<(T, T)> for super::GraphEdge {
    fn from(value: (T, T)) -> Self {
        Self {
            source: value.0.into(),
            target: value.1.into(),
            source_entity: None,
            target_entity: None,
        }
    }
}
