use std::process::Command;

use proto_builder_trait::tonic::BuilderAttributes;

fn main() {
    // tonic_build::configure()
    //     .out_dir("src/pb")
    //     .type_attribute("reservation.ReservationStatus", "#[derive(sqlx::Type)]")
    //     .type_attribute("reservation.ReservationQuery", "#[derive(derive_builder::Builder)]")
    //     .field_attribute("reservation.ReservationQuery.start", "#[builder(setter(into, strip_option))]")
    //     .field_attribute("reservation.ReservationQuery.end", "#[builder(setter(into, strip_option))]")
    //     .compile(&["protos/reservation.proto"], &["protos"])
    //     .unwrap();
    tonic_build::configure()
        .out_dir("src/pb")
        //.type_attribute("reservation.ReservationStatus", "#[derive(sqlx::Type)]")
        .with_sqlx_type(&["reservation.ReservationStatus"])
        .with_derive_builder(&[
            "reservation.ReservationQuery",
            "reservation.ReservationFilter",
        ])
        .with_derive_builder_into(
            "reservation.ReservationQuery",
            &[
                "resource_id",
                "user_id",
                "status",
                "page_size",
                "page",
                "desc",
            ],
        )
        .with_derive_builder_into(
            "reservation.ReservationFilter",
            &["resource_id", "user_id", "status", "desc"],
        )
        .with_derive_builder_option("reservation.ReservationFilter", &["cursor"])
        .with_derive_builder_option("reservation.ReservationQuery", &["start", "end"])
        .with_type_attributes(
            &["reservation.ReservationFilter"],
            &[r#"#[builder(build_fn(name = "private_build"))]"#],
        )
        .with_field_attributes(
            &["reservation.ReservationFilter.page_size"],
            &["#[builder(setter(into), default = \"10\")]"],
        )
        .compile(&["protos/reservation.proto"], &["protos"])
        .unwrap();
    //fs::remove_file("src/pb/google.protobuf.rs").unwrap();
    Command::new("cargo").args(["fmt"]).output().unwrap();
    println!("cargo:return-if-changed=protos/reservation.proto");
}

// trait BuilderExt {
//     fn with_sql_type(self, paths: &[&str]) -> Self;
//     fn with_builder(self, paths: &[&str]) -> Self;
//     fn with_builder_into(self, path: &str, fields: &[&str]) -> Self;
//     fn with_builder_option(self, path: &str, fields: &[&str]) -> Self;
// }

// impl BuilderExt for Builder {
//     fn with_sql_type(self, paths: &[&str]) -> Self {
//         paths.iter().fold(self, |acc, path| {
//             acc.type_attribute(path, "#[derive(sqlx::Type)]")
//         })
//     }
//     fn with_builder(self, paths: &[&str]) -> Self {
//         paths.iter().fold(self, |acc, path| {
//             acc.type_attribute(path, "#[derive(derive_builder::Builder)]")
//         })
//     }

//     fn with_builder_into(self, path: &str, fields: &[&str]) -> Self {
//         fields.iter().fold(self, |acc, field| {
//             acc.field_attribute(
//                 format!("{}.{}", path, field),
//                 "#[builder(setter(into), default)]",
//             )
//         })
//     }
//     fn with_builder_option(self, path: &str, fields: &[&str]) -> Self {
//         fields.iter().fold(self, |acc, field| {
//             acc.field_attribute(
//                 format!("{}.{}", path, field),
//                 "#[builder(setter(into, strip_option))]",
//             )
//         })
//     }
// }
