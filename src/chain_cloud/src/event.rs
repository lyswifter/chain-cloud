
use context::{metadata, util};
use ic_cdk::export::candid::Nat;
use ic_cdk::export::Principal;
use ic_cdk::storage;
use std::collections::HashMap;
use crate::types::{Db, Snapshot};
static mut count: Option<Nat> = None;
static mut Event: Vec<metadata::Metadata> = vec![];
type CanisterEvent = HashMap<Principal, Vec<usize>>;
type CallerEvent = HashMap<Principal, Vec<usize>>;

pub async fn create_event(metadata: metadata::Metadata) -> Result<(), String> {
    let position: usize;
    unsafe {
        Event.push(metadata.clone());
        position = Event.len() - 1;
    }
    let canister_event = storage::get_mut::<CanisterEvent>();
    if !canister_event.contains_key(&metadata.canister) {
        canister_event.insert(metadata.canister.clone(), vec![position]);
    }
    let caller_event = storage::get_mut::<CallerEvent>();
    if !caller_event.contains_key(&metadata.caller) {
        caller_event.insert(metadata.caller.clone(), vec![position]);
        return Ok(());
    }

    canister_event
        .get_mut(&metadata.canister)
        .unwrap()
        .push(position);
    caller_event
        .get_mut(&metadata.caller)
        .unwrap()
        .push(position);

    let len = canister_event.get(&metadata.canister).unwrap().len();
    Ok(())
}

pub async fn get_canister_event(
    canister: Principal,
    offset: Nat,
    limit: Nat,
) -> Vec<metadata::Metadata> {
    let offset = util::nat_to_u64(offset).unwrap() as usize;
    let mut limit = util::nat_to_u64(limit).unwrap() as usize;
    if limit > 50 {
        limit = 50;
    }
    let canister_event = storage::get::<CanisterEvent>();
    if !canister_event.contains_key(&canister) {
        return vec![];
    }
    let blucket = canister_event.get(&canister).unwrap();
    if offset > blucket.len() {
        return vec![];
    }
    let mut container: Vec<metadata::Metadata> = vec![];
    unsafe {
        if offset + limit >= blucket.len() {
            for i in blucket[offset..blucket.len()].iter() {
                container.push(Event[*i].clone());
            }
            return container;
        }

        for i in blucket[offset..offset + limit].iter() {
            container.push(Event[*i].clone());
        }
        return container;
    }
}

pub async fn get_caller_event(
    caller: Principal,
    offset: Nat,
    limit: Nat,
) -> Vec<metadata::Metadata> {
    let offset = util::nat_to_u64(offset).unwrap() as usize;
    let mut limit = util::nat_to_u64(limit).unwrap() as usize;
    if limit > 50 {
        limit = 50;
    }
    let caller_event = storage::get::<CallerEvent>();
    if !caller_event.contains_key(&caller) {
        return vec![];
    }
    let blucket = caller_event.get(&caller).unwrap();
    if offset > blucket.len() {
        return vec![];
    }
    let mut container: Vec<metadata::Metadata> = vec![];
    unsafe {
        if offset + limit >= blucket.len() {
            for i in blucket[offset..blucket.len()].iter() {
                container.push(Event[*i].clone());
            }
            return container;
        }

        for i in blucket[offset..offset + limit].iter() {
            container.push(Event[*i].clone());
        }
        return container;
    }
}

pub async fn get_last_event(limit: Nat) -> Vec<metadata::Metadata> {
    let limit = util::nat_to_u64(limit).unwrap() as usize;
    unsafe {
        let len = Event.len();
        if len == 0 {
            return vec![];
        }
        if len <= limit {
            return Event.to_vec();
        }
        return Event[len - limit - 1..len].to_vec();
    }
}

pub async fn get_canister_last_event(canister: Principal, limit: Nat) -> Vec<metadata::Metadata> {
    let limit = util::nat_to_u64(limit).unwrap() as usize;
    let canister_event = storage::get::<CanisterEvent>();
    let mut result: Vec<metadata::Metadata> = vec![];
    if !canister_event.contains_key(&canister) {
        return result;
    }
    let event = canister_event.get(&canister).unwrap();
    let len = event.len();
    unsafe {
        if len <= limit {
            for i in event.iter() {
                result.push(Event[*i].clone())
            }
            return result;
        }
        for i in event[len - limit - 1..len].iter() {
            result.push(Event[*i].clone())
        }
        return result;
    }
}

pub async fn get_canister_list() -> Vec<Principal> {
    let mut res: Vec<Principal> = vec![];
    let canister_event = storage::get::<CanisterEvent>();
    for keys in canister_event.keys() {
        res.push(keys.clone());
    }
    return res;
}

pub fn pre_upgrade() {
    let mut canister_event_snapshot: Vec<Snapshot<Vec<usize>>> = vec![];
    let mut caller_event_snapshot: Vec<Snapshot<Vec<usize>>> = vec![];
    let canister_event = storage::get::<CanisterEvent>();
    for (k, v) in canister_event.iter() {
        let snapshot = Snapshot::<Vec<usize>> {
            key: k.clone(),
            value: v.to_vec(),
        };
        canister_event_snapshot.push(snapshot);
    }
    let caller_event = storage::get::<CallerEvent>();

    for (k, v) in caller_event.iter() {
        let snapshot = Snapshot::<Vec<usize>> {
            key: k.clone(),
            value: v.to_vec(),
        };
        caller_event_snapshot.push(snapshot);
    }

    unsafe {
        let db = Db {
            canisterEvent: canister_event_snapshot,
            callerEvent: caller_event_snapshot,
            event: Event.to_vec(),
        };
        storage::stable_save((db,));
    }
    let size = ic_cdk::api::stable::stable_size();
    let size = format!("Current used memory page size: {}", size);
}

pub fn post_update() {
    let db = storage::stable_restore::<(Db,)>().expect("Data recovery failed");
    data_load(db.0);
}

/// Reload data into memory
fn data_load(db: Db) {
    unsafe {
        Event = db.event;
    }
    let canister_event = storage::get_mut::<CanisterEvent>();
    for e in db.canisterEvent.into_iter() {
        canister_event.insert(e.key.clone(), e.value);
    }
    for e in db.callerEvent.into_iter() {
        canister_event.insert(e.key.clone(), e.value);
    }
}
