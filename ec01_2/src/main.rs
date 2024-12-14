use std::fmt::Display;

// const NOTES: &str = "AxBCDDCAxD";
const NOTES: &str = "CCDDDCDBxDxADDxDxDBBDAxDDDxDCCCBDCBBBAABCCABDCABxBADBCABACDCCACCCBAADDAxCCDCDCDCBCxBCCAABBCBADBxxDDABDCBCCCABBCBDAxBCACBABDAAACxDACCBCAADCAAAAxADBBCBCBBBDCDAxDxBDACCCAAAxBACxBAABxCAABBDACADABDCDxDCDCABCDCCDCCDAADDABDCACBAxBABDxCDBADDCBACADCDCDADABAABABAADBDCDBBADAxCDBxCBDxCCBACAACDBCDADADABCDDBBADDBBBABxDBAAAADCDBDxBxACCBBDDAxCBAxDDAAABBDxxADACDBCCDBAABBBBAAABxxDCDxxxBCDDBBCBDDACCDCDDDDBABBDCDCACDAAxAABBCBDDACCAABCDBCACCABDDDBABBxxxBxBCDxCDABDABCCDBBDBBBCABBxCCCBDABACDACCDDABxDDBACDAAxBCDxDADACACBACDADBBACCADCBADCBCDABBxBDCCAxDBDAxAxDBAAABBDACxxBDAxBDDxxBDxxDBABCCBAxxAABDBBACBAAAAABDAADABCDDCxCDDCAABDAABDBACBDBADAAxBBDACBDAxCBAxDCxADDCACABDBACBBBADxxABCCDCADADBCBBADDAAxCBBCCDADADBDABAABCBDAxDCDCDBCDABCAAAADBAAAxCCCAACBBxAABCCADCACABDCCBDBADDxABADxBBAABBCDABDACACDDADAAxxCACCCADxDAABDBCABCBCAxBADABACBBxDBAxCAxxDBDDCxADBABDBAxCACCDABAAACBCBCDACACBABCCCDxAADCABADxDCBAxBABBCADDADCDADCABxxBADDAACCBCDADAAABAxDDDBDCCDACBxxxCDBCDCCCDBBxAxDCBBADACADAxCDADAACBxDCDBBADxCCACDDABBCxBCDADCADBBBABABCCDCBBABDBxCBCBABCBCDDDBDCBBAADDDxCBCAAABCBBBCDxBDCBxACCACCBDDCCDBBBBCABABCCxBBCAxBDCBCDABDDCxBCDCCCBBBACCDBCCxAADADDCxxCBDDxCCxCBDADDDBDDxCBxADDACCCDDxCDBDCCxABACDBBCDCDBACxBBCBBACBADDAAADBCBCCACBABCCDCDxBDCDDCCBBDBDDBBDBDCDAAxBDADCCAxCCDCDCCABCAACDACADxDBADCBBCDACBBBADABAxBxBBBBBCDDCBBBBBADBCCDADDCCAxCAxCBDBBxBxACCDDDABABDBxBBAxDBBCxACDDCDxDBBBxADBADCAADACDBBBCxADxAxADDABADACACBDCxDCCCDxAACBAxBADCBBAAABDxADDxDDCADCACAABDBBDBDBDCxBBADDAxDxBAACACACACDACxADDBABCACxxBDBDDBCADxBDBDABxDxCBACDxDxCABAxCxACADAACDDADCBxCCCACCABDDDxxAABCADAxAxDADCDABDCBADBBDBDBCACCAxDBDxDABACCCABDCACCAxBDDBDADBCDxDDDAABCBBDBBCDCCxCDADDAAAACDCAxDDBBBBCDCDBCDBDCABBxBxAABBCDAxDCBBCxCACADxBDxCBBCACCBAACCDBCDBDAABBDBDxDCCBDxACBDCCDBCCACDDxDDAxADBxxAxxCDBCBCBxCCDCCBACBCADDCADDACABBBxCDBADCAABBDBDxDDCDBCCCBCBDADABABDDBAxADDABABBxxDBxBxBBDCxDDACACABDBDDCxBADBCCBADBDBDDxADCDCCCBDCDAADxDDCADBADDDADxCAxBxCADADDDADADCADDBCDxBADCBBAABDABBBCDACABCxCDACACBDCDABxDDADxBDCxxBBADxABDACADBCDAACDACABBCBBDDDxCBBDCCDABCxDAxBDCAADADxADD";

enum Enemy {
    Empty,
    Ant,
    Beetle,
    Cockroach,
    Dragonfly,
}

impl Enemy {
    fn is_empty(&self) -> bool {
        match self {
            Enemy::Empty => true,
            _ => false,
        }
    }
}

impl Display for Enemy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let enemy = match self {
            Enemy::Empty => "x",
            Enemy::Ant => "A",
            Enemy::Beetle => "B",
            Enemy::Cockroach => "C",
            Enemy::Dragonfly => "D",
        };
        f.write_str(enemy)
    }
}

impl From<&Enemy> for i32 {
    fn from(value: &Enemy) -> Self {
        match value {
            Enemy::Ant => 0,
            Enemy::Beetle => 1,
            Enemy::Cockroach => 3,
            Enemy::Dragonfly => 5,
            Enemy::Empty => 0,
        }
    }
}

impl From<char> for Enemy {
    fn from(value: char) -> Self {
        match value {
            'A' => Enemy::Ant,
            'B' => Enemy::Beetle,
            'C' => Enemy::Cockroach,
            'D' => Enemy::Dragonfly,
            'x' => Enemy::Empty,
            _ => panic!("Invalid enemy"),
        }
    }
}
fn main() {
    let data: Vec<_> = NOTES.chars().collect();

    let potions: i32 = data
        .windows(2)
        .step_by(2)
        .map(|c| {
            let e1: Enemy = c[0].into();
            let e2: Enemy = c[1].into();
            let mut sum = Into::<i32>::into(&e1) + Into::<i32>::into(&e2);
            if !e1.is_empty() && !e2.is_empty() {
                sum = sum + 2;
            }
            println!("{}{} = {}", e1, e2, sum);
            sum
        })
        .sum();

    println!("{} potions needed", potions);
}
