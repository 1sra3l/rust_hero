pub struct Jogador {
    pub nome: String,
    pub equipamento: usize,
    pub vida_total: u8,
    pub vida_atual: u8,
    pub ataque: u8,
    pub defesa: u8,
    pub experiencia: u16
}

pub struct Oponente {
    pub nome: String,
    pub vida_total: u8,
    pub vida_atual: u8,
    pub ataque: u8,
    pub defesa: u8,
    pub experiencia: u16,
}