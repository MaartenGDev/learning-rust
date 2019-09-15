use crate::structs::{Container, State, DesiredContainer, CreatedContainer, ActualState};
use crate::docker_client::{get_running_containers, create_container, start_container, kill_container};
use tokio::prelude::Future;
use crate::errors::FetchError;

pub fn get_missing_containers(desired_state: State) -> impl Future<Item=State, Error=FetchError> {
    get_running_containers_in_context().map(|running_containers| {
        let containers = &running_containers;

        State {
            containers: desired_state.containers.into_iter().filter(|desired_container| {
                !containers.into_iter().any(|running_container|  running_container.image == desired_container.image)
            }).collect()
        }
    })
}

pub fn get_zombie_containers(desired_state: State) -> impl Future<Item=ActualState, Error=FetchError> {
    get_running_containers_in_context().map(|running_containers| {
        let containers = desired_state.containers;

        ActualState {
            containers: running_containers.into_iter().filter(|running_container|{
                !containers.iter().any(|desired_container | desired_container .image == running_container.image)
            }).collect()
        }
    })
}


pub fn stop_container(container: &Container) -> impl Future<Item=bool, Error=FetchError> {
    kill_container(container).map(|has_stopped| has_stopped)
}

pub fn schedule_container(container: &DesiredContainer) -> impl Future<Item=impl Future<Item=CreatedContainer, Error=FetchError>, Error=FetchError> {
    create_container(container)
        .map(|created_container| {
            start_container(&created_container).map(|_| created_container)
        })
}

fn get_running_containers_in_context() -> impl Future<Item=Vec<Container>, Error=FetchError> {
    get_running_containers().map(|containers: Vec<Container>| {
        containers.into_iter().filter(|container| {
            container.labels.contains_key("me.maartedev.simplekube")
        }).collect()
    })
}
