fn main() {
    println!("CFC is adjusting the starting posture...");

    let cfc = CFC {
        router: Router {},
        database: Database {},
    };

    cfc.router.loading();
    cfc.database.loading();
    cfc.run();
}

// cfc is the marathon runner.
struct CFC {
    router: Router,
    database: Database,
}

impl CFC {
    fn run(&self) {
        println!("CFC is running");
    }
}

struct Router {}

impl Router {
    fn loading(&self) {
        println!("CFC database is loading...");
        println!("CFC database has been loaded!");
    }
}

struct Database {}

impl Database {
    fn loading(&self) {
        println!("CFC database is loading...");
        println!("CFC database has been loaded!");
    }
}
