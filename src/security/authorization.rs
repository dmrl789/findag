use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use chrono;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Permission {
    Read,
    Write,
    Execute,
    Admin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub name: String,
    pub permissions: Vec<Permission>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRole {
    pub user_id: String,
    pub role_name: String,
    pub assigned_at: i64,
}

pub struct AuthorizationManager {
    roles: HashMap<String, Role>,
    user_roles: HashMap<String, Vec<UserRole>>,
}

impl AuthorizationManager {
    pub fn new() -> Self {
        Self {
            roles: HashMap::new(),
            user_roles: HashMap::new(),
        }
    }

    pub fn create_role(&mut self, name: &str, permissions: Vec<Permission>) -> Result<(), String> {
        if self.roles.contains_key(name) {
            return Err("Role already exists".to_string());
        }

        let now = chrono::Utc::now().timestamp();
        let role = Role {
            name: name.to_string(),
            permissions,
            created_at: now,
            updated_at: now,
        };

        self.roles.insert(name.to_string(), role);
        Ok(())
    }

    pub fn assign_role(&mut self, user_id: &str, role_name: &str) -> Result<(), String> {
        if !self.roles.contains_key(role_name) {
            return Err("Role does not exist".to_string());
        }

        let user_roles = self.user_roles.entry(user_id.to_string())
            .or_insert_with(Vec::new);

        if user_roles.iter().any(|r| r.role_name == role_name) {
            return Err("User already has this role".to_string());
        }

        let user_role = UserRole {
            user_id: user_id.to_string(),
            role_name: role_name.to_string(),
            assigned_at: chrono::Utc::now().timestamp(),
        };

        user_roles.push(user_role);
        Ok(())
    }

    pub fn has_permission(&self, user_id: &str, permission: &Permission) -> bool {
        if let Some(user_roles) = self.user_roles.get(user_id) {
            for user_role in user_roles {
                if let Some(role) = self.roles.get(&user_role.role_name) {
                    if role.permissions.contains(permission) {
                        return true;
                    }
                }
            }
        }
        false
    }

    pub fn get_user_permissions(&self, user_id: &str) -> Vec<Permission> {
        let mut permissions = Vec::new();
        if let Some(user_roles) = self.user_roles.get(user_id) {
            for user_role in user_roles {
                if let Some(role) = self.roles.get(&user_role.role_name) {
                    permissions.extend(role.permissions.clone());
                }
            }
        }
        permissions
    }
} 