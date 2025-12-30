use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 1. USER TABLOSU
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(User::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(User::Username).string().not_null())
                    .col(ColumnDef::new(User::Email).string().unique_key().not_null())
                    .col(ColumnDef::new(User::PasswordHashed).string().not_null())
                    .col(ColumnDef::new(User::Status).string_len(1).not_null().default("A"))
                    .col(ColumnDef::new(User::Created).timestamp_with_time_zone().default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await?;

        // 2. SEAT TABLOSU (Konser Alanı)
        manager
            .create_table(
                Table::create()
                    .table(Seat::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Seat::Id).integer().not_null().auto_increment().primary_key())
                    .col(ColumnDef::new(Seat::Row).string().not_null()) // Örn: "A"
                    .col(ColumnDef::new(Seat::Number).integer().not_null()) // Örn: 1

                    // Aynı koltuktan (A-1) iki tane olamaz:
                    .index(
                        Index::create()
                            .unique()
                            .name("idx_seat_row_number")
                            .col(Seat::Row)
                            .col(Seat::Number)
                    )
                    .to_owned(),
            )
            .await?;

        // 3. TICKET TABLOSU (Satışlar)
        manager
            .create_table(
                Table::create()
                    .table(Ticket::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Ticket::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Ticket::UserId).uuid().not_null())
                    .col(ColumnDef::new(Ticket::SeatId).integer().not_null()) // Seat.Id integer olduğu için bu da integer
                    .col(ColumnDef::new(Ticket::Cost).decimal().not_null()) // Para birimi için Decimal
                    .col(ColumnDef::new(Ticket::Created).timestamp_with_time_zone().default(Expr::current_timestamp()))

                    // FK: Ticket -> User
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ticket_user")
                            .from(Ticket::Table, Ticket::UserId)
                            .to(User::Table, User::Id)
                    )
                    // FK: Ticket -> Seat
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ticket_seat")
                            .from(Ticket::Table, Ticket::SeatId)
                            .to(Seat::Table, Seat::Id)
                    )
                    // ÖNEMLİ: Bir koltuk sadece bir bilette olabilir (Satıldı garantisi)
                    .index(
                        Index::create()
                            .unique()
                            .name("idx_unique_ticket_seat")
                            .col(Ticket::SeatId)
                    )
                    .to_owned(),
            )
            .await?;

        // 4. SESSIONS TABLOSU
        manager
            .create_table(
                Table::create()
                    .table(Sessions::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Sessions::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Sessions::UserId).uuid().not_null())
                    .col(ColumnDef::new(Sessions::RefreshTokenHash).string().not_null().unique_key())

                    // Revoke durumu
                    .col(ColumnDef::new(Sessions::IsRevoked).boolean().not_null().default(false))

                    // Zamanlar
                    .col(ColumnDef::new(Sessions::CreatedAt).timestamp_with_time_zone().default(Expr::current_timestamp()).not_null())
                    .col(ColumnDef::new(Sessions::LastUsedAt).timestamp_with_time_zone().default(Expr::current_timestamp()).not_null())

                    // Enum'da var ama create_table'da unutulmuştu, ekledim:
                    .col(ColumnDef::new(Sessions::ExpiresAt).timestamp_with_time_zone().not_null())

                    // RevokedAt başlangıçta NULL olmalı (henüz iptal edilmedi)
                    .col(ColumnDef::new(Sessions::RevokedAt).timestamp_with_time_zone().null())

                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_session_user_id")
                            .from(Sessions::Table, Sessions::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Bağımlılık sırasına göre tersten siliyoruz
        manager.drop_table(Table::drop().table(Sessions::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Ticket::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(Seat::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(User::Table).to_owned()).await?;

        Ok(())
    }
}


#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Username,
    Email,
    PasswordHashed,
    Status,
    Created,
}

#[derive(DeriveIden)]
enum Seat {
    Table,
    Id,
    Row,
    Number,
}

#[derive(DeriveIden)]
enum Ticket {
    Table,
    Id,
    UserId,
    SeatId,
    Cost,
    Created,
}

#[derive(DeriveIden)]
enum Sessions {
    Table,
    Id,
    UserId,
    RefreshTokenHash,
    IsRevoked,
    CreatedAt,
    ExpiresAt,
    LastUsedAt,
    RevokedAt,
}