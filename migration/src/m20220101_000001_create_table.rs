use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // La fonction `up` est exécutée quand tu APPLIQUES la migration (création)
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    // L'ID est un UUID, clé primaire, non nul
                    .col(ColumnDef::new(Users::Id).uuid().not_null().primary_key())
                    // Username et Email sont uniques (on ne veut pas de doublons)
                    .col(ColumnDef::new(Users::Username).string().not_null().unique_key())
                    .col(ColumnDef::new(Users::Email).string().not_null().unique_key())
                    // Les autres champs classiques
                    .col(ColumnDef::new(Users::Firstname).string().not_null())
                    .col(ColumnDef::new(Users::Lastname).string().not_null())
                    // On stocke le hash du mot de passe
                    .col(ColumnDef::new(Users::PasswordHash).string().not_null())
                    .to_owned(),
            )
            .await
    }

    // La fonction `down` est exécutée quand tu ANNULES la migration (rollback)
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
    }
}

// Cet enum sert de "dictionnaire" pour éviter les fautes de frappe.
// SeaORM va transformer `PasswordHash` en `password_hash` automatiquement dans la BDD.
#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Username,
    Email,
    Firstname,
    Lastname,
    PasswordHash,
}