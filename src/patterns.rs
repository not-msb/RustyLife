use crate::Field;

pub fn glider(mut field: Field) -> Field {
    field[5][5] = 1;
    field[6][6] = 1;
    field[7][4] = 1;
    field[7][5] = 1;
    field[7][6] = 1;

    field
}

pub fn glider_generator(mut field: Field) -> Field {
    field[5][5] = 1;
    field[5][6] = 1;
    field[6][5] = 1;
    field[6][6] = 1;
    
    field[5][15] = 1;
    field[6][15] = 1;
    field[7][15] = 1;
    field[4][16] = 1;
    field[8][16] = 1;
    field[3][17] = 1;
    field[3][18] = 1;
    field[9][17] = 1;
    field[9][18] = 1;
    
    field[6][19] = 1;

    field[4][20] = 1;
    field[8][20] = 1;
    field[5][21] = 1;
    field[6][21] = 1;
    field[7][21] = 1;
    field[6][22] = 1;

    field[3][25] = 1;
    field[4][25] = 1;
    field[5][25] = 1;
    field[3][26] = 1;
    field[4][26] = 1;
    field[5][26] = 1;
    field[2][27] = 1;
    field[6][27] = 1;

    field[1][29] = 1;
    field[2][29] = 1;
    field[6][29] = 1;
    field[7][29] = 1;

    field[3][39] = 1;
    field[4][39] = 1;
    field[3][40] = 1;
    field[4][40] = 1;

    field
}