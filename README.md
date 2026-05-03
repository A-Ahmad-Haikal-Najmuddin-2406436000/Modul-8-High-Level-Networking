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
    
    6. In the MyPaymentService implementation, what additional steps might be necessary to handle more complex payment processing logic?
    
    7. What impact does the adoption of gRPC as a communication protocol have on the overall architecture and design of distributed systems, particularly in terms of interoperability with other technologies and platforms?
    
    8. What are the advantages and disadvantages of using HTTP/2, the underlying protocol for gRPC, compared to HTTP/1.1 or HTTP/1.1 with WebSocket for REST APIs?
    
    9. How does the request-response model of REST APIs contrast with the bidirectional streaming capabilities of gRPC in terms of real-time communication and responsiveness?
    
    10. What are the implications of the schema-based approach of gRPC, using Protocol Buffers, compared to the more flexible, schema-less nature of JSON in REST API payloads?
