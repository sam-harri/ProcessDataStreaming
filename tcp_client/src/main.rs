use heat_exchanger::HeatExchanger;
use distillation_column::DistillationColumn;
use tokio::task;

mod producer;
mod schema_loader;
mod heat_exchanger;
mod distillation_column;

#[tokio::main]
async fn main() {
    let exchanger_handle = task::spawn(async move {
        let heat_exchanger = HeatExchanger::new();
        heat_exchanger.produce_data("hx-topic").await;
    });

    let column_handle = task::spawn(async move {
        let distillation_column = DistillationColumn::new();
        distillation_column.produce_data("distillation-topic").await;
    });

    exchanger_handle.await.unwrap();
    column_handle.await.unwrap();
}
