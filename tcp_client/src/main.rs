// main.rs

use heat_exchanger::HeatExchanger;
use tokio::task;

mod producer;
mod schema_loader;
mod heat_exchanger;
// mod distillation_column; // Uncomment if you have this module

#[tokio::main]
async fn main() {
    let heat_exchanger = HeatExchanger::new();
    let topic = "test-topic";

    let exchanger_handle = task::spawn(async move {
        heat_exchanger.produce_data(topic).await;
    });

    // If you have other units, instantiate and spawn them here
    // let distillation_column = DistillationColumn::new();
    // let column_handle = task::spawn(async move {
    //     distillation_column.produce_data("distillation-topic").await;
    // });

    exchanger_handle.await.unwrap();
    // column_handle.await.unwrap();
}
