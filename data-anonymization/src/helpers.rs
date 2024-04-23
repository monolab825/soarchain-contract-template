#[cfg(test)]
pub mod test {
    use std::marker::PhantomData;
    use crate::execute;
    use crate::error::ContractError;
    use cosmwasm_std::testing::{
        MockApi, MockQuerier, MockStorage,
    };
    use cosmwasm_std::OwnedDeps;

    pub fn our_mock_dependencies() -> OwnedDeps<MockStorage, MockApi, MockQuerier> {
        OwnedDeps {
            storage: MockStorage::default(),
            api: MockApi::default(),
            querier: MockQuerier::default(),
            custom_query_type: PhantomData,
        }
    }


    pub fn setup() -> Result<OwnedDeps<MockStorage, MockApi, MockQuerier>, ContractError> {

        let deps = our_mock_dependencies();
        let operations = vec![
            execute::ConnectionInput {
                operation: execute::FullOperation::Set,
                source_chain: "soarchain".to_string(),
            },
        ];
        execute::connection_operations(operations)?;

        Ok(deps)
    }

}
