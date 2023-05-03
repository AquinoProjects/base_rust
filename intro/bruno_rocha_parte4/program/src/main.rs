//É alocada na memória STATIC do computador
static _Y: u32 = 13;

fn main() {
    //É Alocada na memória STACK
    let x = 5;
    let z = true;
    let numbers = [1,2,3];
    //
    //Valores como input, retorno de apis ou consulta no banco são alocadas na memória HEAP
    //let users = get_users();
}
