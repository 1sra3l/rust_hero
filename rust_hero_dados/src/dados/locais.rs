use super::inimigos::Inimigos;
use crate::structs::local::Local;

#[derive(Copy, Clone)]
pub enum Locais {
    Nenhum,
    Cidade,
    Floresta,
}

impl Locais {
    // Id dos locais
    pub const fn get_id(self) -> usize {
        self as usize
    }

    // Monta o Local
    pub const fn get_local(self) -> Local {
        match self {
            Locais::Nenhum => Local {
                id: self.get_id(),
                nome: "",
                inimigos: [
                    Inimigos::Nenhum,
                    Inimigos::Nenhum,
                    Inimigos::Nenhum,
                    Inimigos::Nenhum,
                ],
            },
            Locais::Cidade => Local {
                id: self.get_id(),
                nome: "Cidade",
                inimigos: [
                    Inimigos::Nenhum,
                    Inimigos::Nenhum,
                    Inimigos::Nenhum,
                    Inimigos::Nenhum,
                ],
            },
            Locais::Floresta => Local {
                id: self.get_id(),
                nome: "Floresta",
                inimigos: [
                    Inimigos::Rato,
                    Inimigos::Coelho,
                    Inimigos::Cobra,
                    Inimigos::Lobo,
                ],
            },
        }
    }
}

pub const LOCAIS: &[Locais] = &[Locais::Nenhum, Locais::Cidade, Locais::Floresta];
