// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]


// interface

pub trait GrafeasV1Beta1 {
    fn get_occurrence(&self, o: ::grpc::RequestOptions, p: super::grafeas::GetOccurrenceRequest) -> ::grpc::SingleResponse<super::grafeas::Occurrence>;

    fn list_occurrences(&self, o: ::grpc::RequestOptions, p: super::grafeas::ListOccurrencesRequest) -> ::grpc::SingleResponse<super::grafeas::ListOccurrencesResponse>;

    fn delete_occurrence(&self, o: ::grpc::RequestOptions, p: super::grafeas::DeleteOccurrenceRequest) -> ::grpc::SingleResponse<super::empty::Empty>;

    fn create_occurrence(&self, o: ::grpc::RequestOptions, p: super::grafeas::CreateOccurrenceRequest) -> ::grpc::SingleResponse<super::grafeas::Occurrence>;

    fn batch_create_occurrences(&self, o: ::grpc::RequestOptions, p: super::grafeas::BatchCreateOccurrencesRequest) -> ::grpc::SingleResponse<super::grafeas::BatchCreateOccurrencesResponse>;

    fn update_occurrence(&self, o: ::grpc::RequestOptions, p: super::grafeas::UpdateOccurrenceRequest) -> ::grpc::SingleResponse<super::grafeas::Occurrence>;

    fn get_occurrence_note(&self, o: ::grpc::RequestOptions, p: super::grafeas::GetOccurrenceNoteRequest) -> ::grpc::SingleResponse<super::grafeas::Note>;

    fn get_note(&self, o: ::grpc::RequestOptions, p: super::grafeas::GetNoteRequest) -> ::grpc::SingleResponse<super::grafeas::Note>;

    fn list_notes(&self, o: ::grpc::RequestOptions, p: super::grafeas::ListNotesRequest) -> ::grpc::SingleResponse<super::grafeas::ListNotesResponse>;

    fn delete_note(&self, o: ::grpc::RequestOptions, p: super::grafeas::DeleteNoteRequest) -> ::grpc::SingleResponse<super::empty::Empty>;

    fn create_note(&self, o: ::grpc::RequestOptions, p: super::grafeas::CreateNoteRequest) -> ::grpc::SingleResponse<super::grafeas::Note>;

    fn batch_create_notes(&self, o: ::grpc::RequestOptions, p: super::grafeas::BatchCreateNotesRequest) -> ::grpc::SingleResponse<super::grafeas::BatchCreateNotesResponse>;

    fn update_note(&self, o: ::grpc::RequestOptions, p: super::grafeas::UpdateNoteRequest) -> ::grpc::SingleResponse<super::grafeas::Note>;

    fn list_note_occurrences(&self, o: ::grpc::RequestOptions, p: super::grafeas::ListNoteOccurrencesRequest) -> ::grpc::SingleResponse<super::grafeas::ListNoteOccurrencesResponse>;

    fn get_vulnerability_occurrences_summary(&self, o: ::grpc::RequestOptions, p: super::grafeas::GetVulnerabilityOccurrencesSummaryRequest) -> ::grpc::SingleResponse<super::grafeas::VulnerabilityOccurrencesSummary>;
}

// client

pub struct GrafeasV1Beta1Client {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
    method_GetOccurrence: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::grafeas::GetOccurrenceRequest, super::grafeas::Occurrence>>,
    method_ListOccurrences: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::grafeas::ListOccurrencesRequest, super::grafeas::ListOccurrencesResponse>>,
    method_DeleteOccurrence: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::grafeas::DeleteOccurrenceRequest, super::empty::Empty>>,
    method_CreateOccurrence: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::grafeas::CreateOccurrenceRequest, super::grafeas::Occurrence>>,
    method_BatchCreateOccurrences: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::grafeas::BatchCreateOccurrencesRequest, super::grafeas::BatchCreateOccurrencesResponse>>,
    method_UpdateOccurrence: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::grafeas::UpdateOccurrenceRequest, super::grafeas::Occurrence>>,
    method_GetOccurrenceNote: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::grafeas::GetOccurrenceNoteRequest, super::grafeas::Note>>,
    method_GetNote: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::grafeas::GetNoteRequest, super::grafeas::Note>>,
    method_ListNotes: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::grafeas::ListNotesRequest, super::grafeas::ListNotesResponse>>,
    method_DeleteNote: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::grafeas::DeleteNoteRequest, super::empty::Empty>>,
    method_CreateNote: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::grafeas::CreateNoteRequest, super::grafeas::Note>>,
    method_BatchCreateNotes: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::grafeas::BatchCreateNotesRequest, super::grafeas::BatchCreateNotesResponse>>,
    method_UpdateNote: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::grafeas::UpdateNoteRequest, super::grafeas::Note>>,
    method_ListNoteOccurrences: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::grafeas::ListNoteOccurrencesRequest, super::grafeas::ListNoteOccurrencesResponse>>,
    method_GetVulnerabilityOccurrencesSummary: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::grafeas::GetVulnerabilityOccurrencesSummaryRequest, super::grafeas::VulnerabilityOccurrencesSummary>>,
}

impl ::grpc::ClientStub for GrafeasV1Beta1Client {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        GrafeasV1Beta1Client {
            grpc_client: grpc_client,
            method_GetOccurrence: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/grafeas.v1beta1.GrafeasV1Beta1/GetOccurrence".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ListOccurrences: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/grafeas.v1beta1.GrafeasV1Beta1/ListOccurrences".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_DeleteOccurrence: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/grafeas.v1beta1.GrafeasV1Beta1/DeleteOccurrence".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_CreateOccurrence: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/grafeas.v1beta1.GrafeasV1Beta1/CreateOccurrence".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_BatchCreateOccurrences: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/grafeas.v1beta1.GrafeasV1Beta1/BatchCreateOccurrences".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_UpdateOccurrence: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/grafeas.v1beta1.GrafeasV1Beta1/UpdateOccurrence".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_GetOccurrenceNote: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/grafeas.v1beta1.GrafeasV1Beta1/GetOccurrenceNote".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_GetNote: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/grafeas.v1beta1.GrafeasV1Beta1/GetNote".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ListNotes: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/grafeas.v1beta1.GrafeasV1Beta1/ListNotes".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_DeleteNote: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/grafeas.v1beta1.GrafeasV1Beta1/DeleteNote".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_CreateNote: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/grafeas.v1beta1.GrafeasV1Beta1/CreateNote".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_BatchCreateNotes: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/grafeas.v1beta1.GrafeasV1Beta1/BatchCreateNotes".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_UpdateNote: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/grafeas.v1beta1.GrafeasV1Beta1/UpdateNote".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ListNoteOccurrences: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/grafeas.v1beta1.GrafeasV1Beta1/ListNoteOccurrences".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_GetVulnerabilityOccurrencesSummary: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/grafeas.v1beta1.GrafeasV1Beta1/GetVulnerabilityOccurrencesSummary".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }
}

impl GrafeasV1Beta1 for GrafeasV1Beta1Client {
    fn get_occurrence(&self, o: ::grpc::RequestOptions, p: super::grafeas::GetOccurrenceRequest) -> ::grpc::SingleResponse<super::grafeas::Occurrence> {
        self.grpc_client.call_unary(o, p, self.method_GetOccurrence.clone())
    }

    fn list_occurrences(&self, o: ::grpc::RequestOptions, p: super::grafeas::ListOccurrencesRequest) -> ::grpc::SingleResponse<super::grafeas::ListOccurrencesResponse> {
        self.grpc_client.call_unary(o, p, self.method_ListOccurrences.clone())
    }

    fn delete_occurrence(&self, o: ::grpc::RequestOptions, p: super::grafeas::DeleteOccurrenceRequest) -> ::grpc::SingleResponse<super::empty::Empty> {
        self.grpc_client.call_unary(o, p, self.method_DeleteOccurrence.clone())
    }

    fn create_occurrence(&self, o: ::grpc::RequestOptions, p: super::grafeas::CreateOccurrenceRequest) -> ::grpc::SingleResponse<super::grafeas::Occurrence> {
        self.grpc_client.call_unary(o, p, self.method_CreateOccurrence.clone())
    }

    fn batch_create_occurrences(&self, o: ::grpc::RequestOptions, p: super::grafeas::BatchCreateOccurrencesRequest) -> ::grpc::SingleResponse<super::grafeas::BatchCreateOccurrencesResponse> {
        self.grpc_client.call_unary(o, p, self.method_BatchCreateOccurrences.clone())
    }

    fn update_occurrence(&self, o: ::grpc::RequestOptions, p: super::grafeas::UpdateOccurrenceRequest) -> ::grpc::SingleResponse<super::grafeas::Occurrence> {
        self.grpc_client.call_unary(o, p, self.method_UpdateOccurrence.clone())
    }

    fn get_occurrence_note(&self, o: ::grpc::RequestOptions, p: super::grafeas::GetOccurrenceNoteRequest) -> ::grpc::SingleResponse<super::grafeas::Note> {
        self.grpc_client.call_unary(o, p, self.method_GetOccurrenceNote.clone())
    }

    fn get_note(&self, o: ::grpc::RequestOptions, p: super::grafeas::GetNoteRequest) -> ::grpc::SingleResponse<super::grafeas::Note> {
        self.grpc_client.call_unary(o, p, self.method_GetNote.clone())
    }

    fn list_notes(&self, o: ::grpc::RequestOptions, p: super::grafeas::ListNotesRequest) -> ::grpc::SingleResponse<super::grafeas::ListNotesResponse> {
        self.grpc_client.call_unary(o, p, self.method_ListNotes.clone())
    }

    fn delete_note(&self, o: ::grpc::RequestOptions, p: super::grafeas::DeleteNoteRequest) -> ::grpc::SingleResponse<super::empty::Empty> {
        self.grpc_client.call_unary(o, p, self.method_DeleteNote.clone())
    }

    fn create_note(&self, o: ::grpc::RequestOptions, p: super::grafeas::CreateNoteRequest) -> ::grpc::SingleResponse<super::grafeas::Note> {
        self.grpc_client.call_unary(o, p, self.method_CreateNote.clone())
    }

    fn batch_create_notes(&self, o: ::grpc::RequestOptions, p: super::grafeas::BatchCreateNotesRequest) -> ::grpc::SingleResponse<super::grafeas::BatchCreateNotesResponse> {
        self.grpc_client.call_unary(o, p, self.method_BatchCreateNotes.clone())
    }

    fn update_note(&self, o: ::grpc::RequestOptions, p: super::grafeas::UpdateNoteRequest) -> ::grpc::SingleResponse<super::grafeas::Note> {
        self.grpc_client.call_unary(o, p, self.method_UpdateNote.clone())
    }

    fn list_note_occurrences(&self, o: ::grpc::RequestOptions, p: super::grafeas::ListNoteOccurrencesRequest) -> ::grpc::SingleResponse<super::grafeas::ListNoteOccurrencesResponse> {
        self.grpc_client.call_unary(o, p, self.method_ListNoteOccurrences.clone())
    }

    fn get_vulnerability_occurrences_summary(&self, o: ::grpc::RequestOptions, p: super::grafeas::GetVulnerabilityOccurrencesSummaryRequest) -> ::grpc::SingleResponse<super::grafeas::VulnerabilityOccurrencesSummary> {
        self.grpc_client.call_unary(o, p, self.method_GetVulnerabilityOccurrencesSummary.clone())
    }
}

// server

pub struct GrafeasV1Beta1Server;


impl GrafeasV1Beta1Server {
    pub fn new_service_def<H : GrafeasV1Beta1 + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/grafeas.v1beta1.GrafeasV1Beta1",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/grafeas.v1beta1.GrafeasV1Beta1/GetOccurrence".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_occurrence(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/grafeas.v1beta1.GrafeasV1Beta1/ListOccurrences".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.list_occurrences(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/grafeas.v1beta1.GrafeasV1Beta1/DeleteOccurrence".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.delete_occurrence(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/grafeas.v1beta1.GrafeasV1Beta1/CreateOccurrence".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.create_occurrence(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/grafeas.v1beta1.GrafeasV1Beta1/BatchCreateOccurrences".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.batch_create_occurrences(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/grafeas.v1beta1.GrafeasV1Beta1/UpdateOccurrence".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.update_occurrence(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/grafeas.v1beta1.GrafeasV1Beta1/GetOccurrenceNote".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_occurrence_note(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/grafeas.v1beta1.GrafeasV1Beta1/GetNote".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_note(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/grafeas.v1beta1.GrafeasV1Beta1/ListNotes".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.list_notes(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/grafeas.v1beta1.GrafeasV1Beta1/DeleteNote".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.delete_note(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/grafeas.v1beta1.GrafeasV1Beta1/CreateNote".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.create_note(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/grafeas.v1beta1.GrafeasV1Beta1/BatchCreateNotes".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.batch_create_notes(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/grafeas.v1beta1.GrafeasV1Beta1/UpdateNote".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.update_note(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/grafeas.v1beta1.GrafeasV1Beta1/ListNoteOccurrences".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.list_note_occurrences(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/grafeas.v1beta1.GrafeasV1Beta1/GetVulnerabilityOccurrencesSummary".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_vulnerability_occurrences_summary(o, p))
                    },
                ),
            ],
        )
    }
}
