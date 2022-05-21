//
// ────────────────────────────────────────────────────────── I ──────────
//   :::::: S E R I A L I Z E : :  :   :    :     :        :          :
// ────────────────────────────────────────────────────────────────────
//

use serde::{
	ser::{Serialize, SerializeStruct, Serializer},
	Deserialize,
};

use crate::DynMap;

impl<'a> Serialize for DynMap<'a> {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let mut state = serializer.serialize_struct("DynMap", self.len())?;

		state.serialize_field("keys", &self.keys)?;

		state.serialize_field("values", &self.values)?;

		state.end()
	}
}

//
// ────────────────────────────────────────────────────────────── I ──────────
//   :::::: D E S E R I A L I Z E : :  :   :    :     :        :          :
// ────────────────────────────────────────────────────────────────────────
//

use serde::de::{self, Deserializer, MapAccess, Visitor};

impl<'de> Deserialize<'de> for DynMap<'de> {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		#[derive(Deserialize)]
		#[serde(field_identifier, rename_all = "lowercase")]
		enum Field {
			Keys,
			Values,
		}

		struct DynMapVisitor;

		impl<'de> Visitor<'de> for DynMapVisitor {
			type Value = DynMap<'de>;
			fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
				formatter.write_str("struct DynMap")
			}

			fn visit_map<V>(self, mut map: V) -> Result<DynMap<'de>, V::Error>
			where
				V: MapAccess<'de>,
			{
				let mut keys = None;
				let mut values = None;

				while let Some(key) = map.next_key()? {
					match key {
						Field::Keys => {
							if keys.is_some() {
								return Err(de::Error::duplicate_field("keys"));
							}
							keys = Some(map.next_value()?);
						}
						Field::Values => {
							if values.is_some() {
								return Err(de::Error::duplicate_field("values"));
							}
							values = Some(map.next_value()?);
						}
					}
				}

				let keys = keys.ok_or_else(|| de::Error::missing_field("keys"))?;

				let values = values.ok_or_else(|| de::Error::missing_field("values"))?;

				Ok(DynMap { keys, values })
			}
		}

		const FIELDS: &'static [&'static str] = &["keys", "values"];

		deserializer.deserialize_struct("DynMap", FIELDS, DynMapVisitor)
	}
}

//
// ──────────────────────────────────────────────── I ──────────
//   :::::: J S O N : :  :   :    :     :        :          :
// ──────────────────────────────────────────────────────────
//

// TODO: Implement JSON serialization
// TODO: Implement RMP serialization (More efficient than JSON)