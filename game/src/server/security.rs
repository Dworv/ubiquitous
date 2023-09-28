pub type SkillArray = [f32; 4];

#[derive(Debug)]
pub struct ServerSecurity {
    pub skill_req: SkillArray,
    pub skill_req_root: SkillArray,
    pub state: SecurityState
}

#[derive(Debug)]
pub enum SecurityState {
    Secure,
    Compromised,
    CompromisedRoot
}

impl ServerSecurity {
    pub fn can_compromise(&self, attack: &AttackInfo) -> bool {
        use AttackKind::*;
        match attack.kind {
            Password => attack.skill >= self.skill_req[0],
            ProtoManip => attack.skill >= self.skill_req[1],
            Impersonation => attack.skill >= self.skill_req[2],
            Collision => attack.skill >= self.skill_req[3],
        }
    }

    pub fn can_compromise_root(&self, attack: &AttackInfo) -> bool {
        use AttackKind::*;
        match attack.kind {
            Password => attack.skill >= self.skill_req_root[0],
            ProtoManip => attack.skill >= self.skill_req_root[1],
            Impersonation => attack.skill >= self.skill_req_root[2],
            Collision => attack.skill >= self.skill_req_root[3],
        }
    }
}

pub struct AttackInfo {
    pub kind: AttackKind,
    pub skill: f32,
}

pub enum AttackKind {
    Password,
    ProtoManip,
    Impersonation,
    Collision,
}
