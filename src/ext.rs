use crate::Alarm;
use crate::AlarmKind;

pub trait ResultAlarmEx<T, K: AlarmKind> {
    fn into_context(self, kind: K) -> Result<T, Alarm<K>>;
    fn into_internal(self, kind: K) -> Result<T, Alarm<K>>;
}

impl<T, E, K> ResultAlarmEx<T, K> for Result<T, E>
where
    E: std::error::Error + 'static,
    K: AlarmKind,
{
    fn into_context(self, kind: K) -> Result<T, Alarm<K>> {
        self.map_err(|error| Alarm::new(kind).with_context(error))
    }
    fn into_internal(self, kind: K) -> Result<T, Alarm<K>> {
        self.map_err(|error| Alarm::new(kind).with_internal(error))
    }
}
