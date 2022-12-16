use std::process::Command;

use tonic_build::Builder;

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
        .with_sql_type(&["reservation.ReservationStatus"])
        .with_builder(&[
            "reservation.ReservationQuery",
            "reservation.ReservationFilter",
        ])
        .with_builder_into(
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
        .with_builder_into(
            "reservation.ReservationFilter",
            &[
                "resource_id",
                "user_id",
                "status",
                "page_size",
                "cursor",
                "desc",
            ],
        )
        .with_builder_option("reservation.ReservationQuery", &["start", "end"])
        .compile(&["protos/reservation.proto"], &["protos"])
        .unwrap();
    //fs::remove_file("src/pb/google.protobuf.rs").unwrap();
    Command::new("cargo").args(["fmt"]).output().unwrap();
    println!("cargo:return-if-changed=protos/reservation.proto");
}

trait BuilderExt {
    fn with_sql_type(self, paths: &[&str]) -> Self;
    fn with_builder(self, paths: &[&str]) -> Self;
    fn with_builder_into(self, path: &str, fields: &[&str]) -> Self;
    fn with_builder_option(self, path: &str, fields: &[&str]) -> Self;
}

impl BuilderExt for Builder {
    fn with_sql_type(self, paths: &[&str]) -> Self {
        paths.iter().fold(self, |acc, path| {
            acc.type_attribute(path, "#[derive(sqlx::Type)]")
        })
    }
    fn with_builder(self, paths: &[&str]) -> Self {
        paths.iter().fold(self, |acc, path| {
            acc.type_attribute(path, "#[derive(derive_builder::Builder)]")
        })
    }

    fn with_builder_into(self, path: &str, fields: &[&str]) -> Self {
        fields.iter().fold(self, |acc, field| {
            acc.field_attribute(
                format!("{}.{}", path, field),
                "#[builder(setter(into), default)]",
            )
        })
    }
    fn with_builder_option(self, path: &str, fields: &[&str]) -> Self {
        fields.iter().fold(self, |acc, field| {
            acc.field_attribute(
                format!("{}.{}", path, field),
                "#[builder(setter(into, strip_option))]",
            )
        })
    }
}
