# Reflection
    1. What are the key differences between unary, server streaming, and bi-directional streaming RPC (Remote Procedure Call) methods, and in what scenarios would each be most suitable?
    A. Unary RPC= User mengirim satu request dan menunggu satu response dari server yang dikirimi request tersebut. Mirip dengan REST standar.
        - Skenario: Unary RPC cocok untuk mengambil data tunggal, seperti profil, form yang sederhana, dan operasi CRUD standar yang mana response langsung diperlukan.
    B. Server Streaming= User akan mengirim satu request, dan server merespons dengan aliran pesan yang berurutan.
        - Skenario: Cocok ketika mengunduh file yang ukurannya besar, atau saat mengambil log dari suatu sistem secara real-time.
    C. Bi-directional Streaming= User dan server secara bersamaan membuka aliran data dan bisa membaca atau menulis pesan secara independen dan asinkronus tanpa menunggu satu sama lain.
        - Skenario: Aplikasi chatting, game multiplayer.

    2. What are the potential security considerations involved in implementing a gRPC service in Rust, particularly regarding authentication, authorization, and data encryption?
    - Autentikasi: Karena gRPC tidak menggunakan cookie atau session seperti web pada umumnya, autentikasi biasanya dilakukan menggunakan token, seperti JWT, yang disisipkan ke dalam Metadata. Kita dapat menggunakan Interceptor di tonic untuk memastikan token ini ada pada setiap request.
    - Otorisasi: Setelah pengguna diautentikasi, otorisasi (RBAC - Role Based Access Control) harus diimplementasikan di level layanan (business logic) supaya memastikan user_id memiliki hak untuk dapat mengakses resource yang diminta.
    - Enkripsi: gRPC menggunakan HTTP/2 sehingga penggunaan TLS/SSL sangat dianjurkan yang umumnya diwajibkan oleh env produksi. Di Rust dengan menggunakan tonic, kita harus mengonfigurasi ServerTlsConfig dan memuat sertifikat untuk memastikan data terenkripsi dengan baik.

    3. What are the potential challenges or issues that may arise when handling bidirectional streaming in Rust gRPC, especially in scenarios like chat applications?
    Beberapa tantangan diantaranya, yaitu me-manage state konkurensi karena kit aharus melacak klien yang terhubung ketat karena adanya aturan ownership, penanganan ketika user terputus tiba-tiba dan perlu membersihkan koneksi mereka dari server tanpa terjadi memory leak, dan memastikan pesan dikirim dan diterima dalam urutan yang tepat.

    4. What are the advantages and disadvantages of using the tokio_stream::wrappers::ReceiverStream for streaming responses in Rust gRPC services?
        - Kelebihannya mudah diintegrasikan dengan tokio::sync::mpsc yang memungkinkan untuk melakukan spawn task asinkronus baru yang akan mengerjakan pekerjaan berat di background, dan mengirimkan hasilnya melalui channel.
        - Kekurangannya adalah ada overhead memori dan komputasi karena melibatkan message passing melalui channel alih-alih mengembalikan iterator secara langsung.

    5. In what ways could the Rust gRPC code be structured to facilitate code reuse and modularity, promoting maintainability and extensibility over time?
        - Memisahkan crate, seperti satu crate yang hanya berisi file .proto dan build script.
        - Pemisahan transport & Business logic, contohnya tonic baiknya hanya bertugas sebagai pengurai request gRPC dan membentuk response. Logika bisnis berada di modul ataupun fungsi rust terpisah yang murni, dan mudah untuk dilakukan unit test.
    
    6. In the MyPaymentService implementation, what additional steps might be necessary to handle more complex payment processing logic?
        - Integrasikan dengan database yang berfungsi menyimpan status transaksi.
        - Hubungkan dengan pihak ketiga & Timeout, seperti hubungkan ke layanan payment gateway eksternal, seperti midtrans, atau stripe. Selain itu, lakukan mekanisme timeout untuk mengatasi kagagalan layanan eksternal
        - Tambahkan log dan ID pelacakan supaya tiap transaksi tercatat dengan baik.
    
    7. What impact does the adoption of gRPC as a communication protocol have on the overall architecture and design of distributed systems, particularly in terms of interoperability with other technologies and platforms?
        - Memiliki komunikasi internal antarservice yang sangat ideal karena payload binernya jauh lebih kecil dan serialize/deserialize menjadi lebih cepat.
        - komunikasi antar bahasa, misal layanan pembayaran dengan rust, layanan analitik dengan python semuannya dapat berkomunikasi dengan baik.
    
    8. What are the advantages and disadvantages of using HTTP/2, the underlying protocol for gRPC, compared to HTTP/1.1 or HTTP/1.1 with WebSocket for REST APIs?
        - Kelebihannya HTTP/2 dapat mengirimkan banyak request dan response secara paralel lewat 1 koneksi TCP tunggal. HTTP/1 rentan terhadap satu request lambat akan memblokir antrian lain
        - HTTP/2 menggunakan HPACK untuk mengompresi header, hal ini sangat mengurangi beban bandwidth, dibanding HTTP/1.1 yang mengirimkan header dalam bentuk teks biasa setiap saat.
        - ebSocket memungkinkan komunikasi dua arah, tapi format datanya tidak terstruktur. gRPC menyediakan streaming dua arah yang memiliki schema dan tipe data yang jelas, sehingga lebih aman digunakan untuk sistem kompleks.
    
    9. How does the request-response model of REST APIs contrast with the bidirectional streaming capabilities of gRPC in terms of real-time communication and responsiveness?
        - REST berjalan dengan cara jika jklien ingin data real-time, ia harus melakukan polling, yaitu bertanya ke server "adakah pesan baru?" tiap beberapa saat. Hal tersebut akan buang-buang koneksi jaringan, CPU, dan meningkatnya latensi
        - gRPC berjalan dengan membiarkan koneksi TCP dibarkan terbuka terus-terusan. Saat server ada pesan baru, pesan itu akan dikirimkan langsung ke klien dengan isntan tanpa klien perlu memintanya berkali-kali. Hal ini membuat responsivitas waktu nyata yang sebenarnya.
    
    10. What are the implications of the schema-based approach of gRPC, using Protocol Buffers, compared to the more flexible, schema-less nature of JSON in REST API payloads?
        A. Protobuf
            - Implikasi positifnya validasi tipe dilakukan dengan ketat. Jika server mencoba mengubah tipe suatu data dari integer ke string, kode akan gagal di-compile sehingga mencegah bug runtime.
            - Implikasi negatifnya adalah menjadi tidak human-readable. Kita tidak akan bisa langsung menggunakan curl untuk melihat teks response, tapi harus ada file.proto untuk menerjemahkan data biner yang kita terima.
        B. JSON
            - Implikasi positif: fleksibel, human-readable, dan mudah didebug menggunakan tools, seperti Postman.
            - implikasi negatif: Berpotensi membuat error di produksi jika saja ada satu microservice yang mengubah struktur key JSON tanpa diketahui. Selain itu, ukuruan payload jauh lebih besar dikarenakan berbentuk teks penuh.
