use tokio::sync::mpsc::error::SendError;

#[derive(thiserror::Error, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum DeviceMonitorError {
    #[error(transparent)]
    FromMqttClient(#[from] mqtt_channel::MqttError),

    #[error(transparent)]
    FromInvalidCollectdMeasurement(#[from] crate::collectd::collectd::CollectdError),

    #[error(transparent)]
    FromInvalidThinEdgeJson(#[from] thin_edge_json::group::MeasurementGrouperError),

    #[error(transparent)]
    FromThinEdgeJsonSerializationError(
        #[from] thin_edge_json::serialize::ThinEdgeJsonSerializationError,
    ),

    #[error(transparent)]
    FromBatchingError(#[from] SendError<thin_edge_json::group::MeasurementGrouper>),
}
