pub mod rqt2 {
    pub mod api {
        pub mod v1 {
            tonic::include_proto!("rqt2.api.v1");
            pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("rqt2_descriptor");
        }
    }
}
