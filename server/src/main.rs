use application::services::{
    CreatePersonService,
    UpdatePersonService,
    DeletePersonService,
    DependOnCreatePersonService,
    DependOnUpdatePersonService,
    DependOnDeletePersonService,
};
use application::transfer::person::{
    CreatePersonDto,
    UpdatePersonDto,
    DeletePersonDto,
};
use server::Handler;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let handler = Handler::init().await?;

    let create = CreatePersonDto {
        name: String::from("TEST MAN")
    };

    let created = handler.create_person_service().create(create).await?;

    println!("{:?}", created);

    let update = UpdatePersonDto {
        id: created.id,
        name: String::from("TEST MAN MK.II")
    };

    let updated = handler.update_person_service().update(update).await?;

    println!("{:?}", updated);

    let delete = DeletePersonDto {
        id: updated.id
    };

    let deleted = handler.delete_person_service().delete(delete).await?;

    println!("{:?}", deleted);

    Ok(())
}