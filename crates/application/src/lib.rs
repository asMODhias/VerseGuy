#![allow(clippy::result_unit_err)]

use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Organization {
    pub id: String,
    pub name: String,
    pub tag: String,
    pub description: String,
    pub member_count: usize,
    pub treasury_balance: i64,
}

#[derive(Clone, Debug)]
pub struct Fleet {
    pub id: String,
    pub organization_id: String,
    pub name: String,
    pub ship_count: usize,
    pub total_crew: usize,
}

#[derive(Clone, Debug)]
pub struct Operation {
    pub id: String,
    pub organization_id: String,
    pub name: String,
    pub status: String,
    pub participant_count: usize,
}

#[derive(Clone)]
pub struct ApplicationService {
    orgs: Arc<Mutex<HashMap<String, Organization>>>,
    fleets: Arc<Mutex<HashMap<String, Fleet>>>,
    ops: Arc<Mutex<HashMap<String, Operation>>>,
}

impl ApplicationService {
    pub fn new() -> Self {
        Self {
            orgs: Arc::new(Mutex::new(HashMap::new())),
            fleets: Arc::new(Mutex::new(HashMap::new())),
            ops: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn create_organization(
        &self,
        dto: CreateOrganizationDto,
        _user: String,
    ) -> Result<Organization, ()> {
        if dto.tag.len() > 5 || dto.tag.chars().any(|c| !c.is_ascii_uppercase()) {
            return Err(());
        }
        let id = Uuid::new_v4().to_string();
        let org = Organization {
            id: id.clone(),
            name: dto.name,
            tag: dto.tag,
            description: dto.description,
            member_count: 1,
            treasury_balance: 0,
        };
        let mut guard = self.orgs.lock().map_err(|_| ())?;
        guard.insert(id.clone(), org.clone());
        Ok(org)
    }

    pub fn add_member(&self, dto: AddMemberDto, _user: String) -> Result<(), ()> {
        let mut map = self.orgs.lock().map_err(|_| ())?;
        if let Some(org) = map.get_mut(&dto.organization_id) {
            org.member_count += 1;
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn get_organization(&self, id: &str) -> Result<Organization, ()> {
        let map = self.orgs.lock().map_err(|_| ())?;
        map.get(id).cloned().ok_or(())
    }

    pub fn deposit_funds(&self, dto: TreasuryOperationDto, _user: String) -> Result<(), ()> {
        let mut map = self.orgs.lock().map_err(|_| ())?;
        if let Some(org) = map.get_mut(&dto.organization_id) {
            org.treasury_balance += dto.amount;
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn withdraw_funds(&self, dto: TreasuryOperationDto, _user: String) -> Result<(), ()> {
        let mut map = self.orgs.lock().map_err(|_| ())?;
        if let Some(org) = map.get_mut(&dto.organization_id) {
            if org.treasury_balance < dto.amount {
                return Err(());
            }
            org.treasury_balance -= dto.amount;
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn create_fleet(&self, dto: CreateFleetDto, _user: String) -> Result<Fleet, ()> {
        let id = Uuid::new_v4().to_string();
        let fleet = Fleet {
            id: id.clone(),
            organization_id: dto.organization_id,
            name: dto.name,
            ship_count: 0,
            total_crew: 0,
        };
        let mut guard = self.fleets.lock().map_err(|_| ())?;
        guard.insert(id.clone(), fleet.clone());
        Ok(fleet)
    }

    pub fn add_ship(&self, dto: AddShipDto, _user: String) -> Result<(), ()> {
        let mut map = self.fleets.lock().map_err(|_| ())?;
        if let Some(fleet) = map.get_mut(&dto.fleet_id) {
            fleet.ship_count += 1;
            fleet.total_crew += dto.crew_size as usize;
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn get_fleet(&self, id: &str) -> Result<Fleet, ()> {
        let map = self.fleets.lock().map_err(|_| ())?;
        map.get(id).cloned().ok_or(())
    }

    pub fn create_operation(
        &self,
        dto: CreateOperationDto,
        _user: String,
    ) -> Result<Operation, ()> {
        let id = Uuid::new_v4().to_string();
        let op = Operation {
            id: id.clone(),
            organization_id: dto.organization_id,
            name: dto.name,
            status: "draft".to_string(),
            participant_count: 0,
        };
        let mut guard = self.ops.lock().map_err(|_| ())?;
        guard.insert(id.clone(), op.clone());
        Ok(op)
    }

    pub fn add_participant(&self, dto: AddParticipantDto, _user: String) -> Result<(), ()> {
        let mut map = self.ops.lock().map_err(|_| ())?;
        if let Some(op) = map.get_mut(&dto.operation_id) {
            op.participant_count += 1;
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn start_operation(&self, id: String, _user: String) -> Result<(), ()> {
        let mut map = self.ops.lock().map_err(|_| ())?;
        if let Some(op) = map.get_mut(&id) {
            op.status = "in_progress".to_string();
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn get_operation(&self, id: &str) -> Result<Operation, ()> {
        let map = self.ops.lock().map_err(|_| ())?;
        map.get(id).cloned().ok_or(())
    }
}

// DTOs
#[derive(Clone)]
pub struct CreateOrganizationDto {
    pub name: String,
    pub tag: String,
    pub description: String,
}
#[derive(Clone)]
pub struct AddMemberDto {
    pub organization_id: String,
    pub user_id: String,
}
#[derive(Clone)]
pub struct TreasuryOperationDto {
    pub organization_id: String,
    pub amount: i64,
    pub reason: Option<String>,
}
#[derive(Clone)]
pub struct CreateFleetDto {
    pub organization_id: String,
    pub name: String,
    pub description: String,
}
#[derive(Clone)]
pub struct AddShipDto {
    pub fleet_id: String,
    pub manufacturer: String,
    pub name: String,
    pub variant: Option<String>,
    pub role: String,
    pub owner_id: String,
    pub crew_size: i64,
    pub cargo_capacity: i64,
}
#[derive(Clone)]
pub struct CreateOperationDto {
    pub organization_id: String,
    pub name: String,
    pub description: String,
    pub operation_type: String,
    pub scheduled_start: DateTime<Utc>,
    pub scheduled_end: DateTime<Utc>,
}
#[derive(Clone)]
pub struct AddParticipantDto {
    pub operation_id: String,
    pub user_id: String,
    pub role: String,
}

impl Default for ApplicationService {
    fn default() -> Self {
        Self::new()
    }
}
