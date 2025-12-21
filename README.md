# 🏎️ SeatRush: High-Performance Concurrency Booking Engine

![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange?logo=rust)
![Actix-Web](https://img.shields.io/badge/Framework-Actix_Web-green)
![Postgres](https://img.shields.io/badge/Database-PostgreSQL-blue?logo=postgresql)
![Redis](https://img.shields.io/badge/Cache-Redis-red?logo=redis)
![SeaORM](https://img.shields.io/badge/ORM-SeaORM-lightgrey)

**SeatRush**, yüksek talep gören etkinliklerde (konser, maç vb.) ortaya çıkan "Race Condition" (Yarış Durumu) problemlerini çözmek için tasarlanmış, **Rust** tabanlı, yüksek performanslı bir bilet rezervasyon API'sidir.

Klasik veritabanı işlemlerinin yetersiz kaldığı **"High Concurrency"** (Yüksek Eşzamanlılık) senaryolarında, veri bütünlüğünü sağlamak için **Redis Distributed Locks** ve **ACID Veritabanı İşlemlerini** hibrit bir mimaride kullanır.

---

## 🚀 Projenin Amacı ve Çözdüğü Sorunlar

Aynı milisaniyede 10.000 kişinin "Satın Al" butonuna bastığı bir senaryoda:
1.  **Double Booking (Çifte Rezervasyon):** Aynı koltuğun iki kişiye satılmasını engeller.
2.  **Ghost Reservations:** Ödeme aşamasında terk edilen koltukların belirli bir süre sonra (TTL) otomatik olarak serbest kalmasını sağlar.
3.  **High Throughput:** Actix-Web'in Actor modeli ve Rust'ın sahiplik (ownership) sistemi sayesinde minimum kaynakla maksimum istek karşılar.

---

## 🛠️ Teknoloji Yığını (Tech Stack)

| Alan | Teknoloji | Kullanım Amacı |
| :--- | :--- | :--- |
| **Backend Dili** | Rust 🦀 | Bellek güvenliği, hız ve Thread-Safety. |
| **Web Framework** | Actix-Web | Asenkron HTTP sunucusu. |
| **Veritabanı** | PostgreSQL | Kullanıcı, Bilet ve Kalıcı Veriler. |
| **ORM & Migration** | SeaORM | Type-Safe SQL sorguları ve şema yönetimi. |
| **Distributed Lock** | Redis | Geçici rezervasyon kilitleri (TTL ile). |
| **Altyapı** | Docker & Compose | Servislerin izole çalıştırılması. |

---

## 🏗️ Yazılım Mimarisi (Clean Architecture)

Proje, sorumlulukların net bir şekilde ayrıldığı katmanlı bir mimari kullanır:


```

src/
├── api/            # Controller & Route tanımları (Presentation Layer)
├── domain/         # Entity'ler, DTO'lar ve Enums (Core Layer)
├── service/        # İş Mantığı ve Transaction Yönetimi (Business Layer)
├── repository/     # Veritabanı Erişim Katmanı (Data Access Layer)
├── infrastructure/ # DB Bağlantıları, Redis ve AppState (Infrastructure Layer)
└── utils/          # Hata yönetimi ve yardımcı fonksiyonlar

```

### 🔄 İşleyiş Akışı (Booking Flow)

```mermaid
sequenceDiagram
    participant User
    participant API
    participant Redis
    participant Postgres

    User->>API: POST /reserve/A1 (Koltuk Seç)
    API->>Redis: SETNX seat:A1 (Atomic Lock)
    
    alt Kilit Başarılı (Koltuk Boş)
        Redis-->>API: OK (TTL: 90sn)
        API-->>User: 200 OK (Rezervasyon Token'ı)
    else Kilit Başarısız (Dolu)
        Redis-->>API: Fail
        API-->>User: 409 Conflict (Koltuk Dolu)
    end

    User->>API: POST /buy (Ödeme Yap)
    API->>Redis: GET seat:A1 (Süre Doldu mu?)
    
    alt Süre Geçerli
        API->>Postgres: INSERT INTO tickets (Transaction)
        Postgres-->>API: Success
        API->>Redis: DEL seat:A1 (veya statüyü güncelle)
        API-->>User: 200 OK (Bilet Oluşturuldu)
    else Süre Dolmuş
        API-->>User: 400 Bad Request (Zaman Aşımı)
    end

```

---

## 💾 Veritabanı Şeması

Veri tutarlılığı için sıkı Foreign Key ve Unique Index kuralları uygulanmıştır:

* **Users:** Kullanıcı bilgileri ve durumu (Active/Deleted).
* **Sessions:** Güvenli oturum yönetimi (IP, User-Agent, Revoke özelliği).
* **Seats:** Fiziksel koltuk envanteri (`Row` + `Number` Unique Index).
* **Tickets:** Satılan biletler (`SeatId` üzerinde Unique Index - DB seviyesinde koruma).

---

## ⚙️ Kurulum ve Çalıştırma

Projeyi yerel ortamınızda çalıştırmak için aşağıdaki adımları izleyin.

### Ön Gereksinimler

* Rust & Cargo
* Docker & Docker Compose

### 1. Projeyi Klonlayın

```bash
git clone [https://github.com/kullaniciadi/seat-rush.git](https://github.com/kullaniciadi/seat-rush.git)
cd seat-rush

```

### 2. Ortam Değişkenlerini (.env) Hazırlayın

Ana dizinde `.env` dosyası oluşturun ve aşağıdaki ayarları ekleyin:

```env
DATABASE_URL=postgres://postgres:sifreniz@localhost:5432/seat_rush_db
REDIS_URL=redis://127.0.0.1:6379
RUST_LOG=info

```

### 3. Altyapıyı Başlatın (Docker)

PostgreSQL ve Redis servislerini ayağa kaldırın:

```bash
docker-compose up -d

```

### 4. Veritabanı Migrasyonlarını Çalıştırın

SeaORM ile tabloları oluşturun:

```bash
cargo run -p migration -- up

```

### 5. Sunucuyu Başlatın

```bash
cargo run

```

Sunucu `http://127.0.0.1:8080` adresinde çalışmaya başlayacaktır.

---

## 🧪 Geliştirme Notları

* **Migration Reset:** Veritabanını tamamen sıfırlamak için: `cargo run -p migration -- refresh`
* **Entity Generation:** DB şeması değişirse Entity dosyalarını güncellemek için:
```bash
sea-orm-cli generate entity -u postgres://postgres:sifreniz@localhost:5432/seat_rush_db -o src/domain/entities --with-serde both

```



---

## 📄 Lisans

Bu proje [MIT](https://www.google.com/search?q=LICENSE) lisansı ile lisanslanmıştır.

```

```
