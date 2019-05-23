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

pub trait Projects {
    fn create_project(&self, o: ::grpc::RequestOptions, p: super::project::CreateProjectRequest) -> ::grpc::SingleResponse<super::project::Project>;

    fn get_project(&self, o: ::grpc::RequestOptions, p: super::project::GetProjectRequest) -> ::grpc::SingleResponse<super::project::Project>;

    fn list_projects(&self, o: ::grpc::RequestOptions, p: super::project::ListProjectsRequest) -> ::grpc::SingleResponse<super::project::ListProjectsResponse>;

    fn delete_project(&self, o: ::grpc::RequestOptions, p: super::project::DeleteProjectRequest) -> ::grpc::SingleResponse<super::empty::Empty>;
}

// client

pub struct ProjectsClient {
    grpc_client: ::std::sync::Arc<::grpc::Client>,
    method_CreateProject: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::project::CreateProjectRequest, super::project::Project>>,
    method_GetProject: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::project::GetProjectRequest, super::project::Project>>,
    method_ListProjects: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::project::ListProjectsRequest, super::project::ListProjectsResponse>>,
    method_DeleteProject: ::std::sync::Arc<::grpc::rt::MethodDescriptor<super::project::DeleteProjectRequest, super::empty::Empty>>,
}

impl ::grpc::ClientStub for ProjectsClient {
    fn with_client(grpc_client: ::std::sync::Arc<::grpc::Client>) -> Self {
        ProjectsClient {
            grpc_client: grpc_client,
            method_CreateProject: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/grafeas.v1beta1.project.Projects/CreateProject".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_GetProject: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/grafeas.v1beta1.project.Projects/GetProject".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_ListProjects: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/grafeas.v1beta1.project.Projects/ListProjects".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
            method_DeleteProject: ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                name: "/grafeas.v1beta1.project.Projects/DeleteProject".to_string(),
                streaming: ::grpc::rt::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }
}

impl Projects for ProjectsClient {
    fn create_project(&self, o: ::grpc::RequestOptions, p: super::project::CreateProjectRequest) -> ::grpc::SingleResponse<super::project::Project> {
        self.grpc_client.call_unary(o, p, self.method_CreateProject.clone())
    }

    fn get_project(&self, o: ::grpc::RequestOptions, p: super::project::GetProjectRequest) -> ::grpc::SingleResponse<super::project::Project> {
        self.grpc_client.call_unary(o, p, self.method_GetProject.clone())
    }

    fn list_projects(&self, o: ::grpc::RequestOptions, p: super::project::ListProjectsRequest) -> ::grpc::SingleResponse<super::project::ListProjectsResponse> {
        self.grpc_client.call_unary(o, p, self.method_ListProjects.clone())
    }

    fn delete_project(&self, o: ::grpc::RequestOptions, p: super::project::DeleteProjectRequest) -> ::grpc::SingleResponse<super::empty::Empty> {
        self.grpc_client.call_unary(o, p, self.method_DeleteProject.clone())
    }
}

// server

pub struct ProjectsServer;


impl ProjectsServer {
    pub fn new_service_def<H : Projects + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::rt::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::rt::ServerServiceDefinition::new("/grafeas.v1beta1.project.Projects",
            vec![
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/grafeas.v1beta1.project.Projects/CreateProject".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.create_project(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/grafeas.v1beta1.project.Projects/GetProject".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.get_project(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/grafeas.v1beta1.project.Projects/ListProjects".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.list_projects(o, p))
                    },
                ),
                ::grpc::rt::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::rt::MethodDescriptor {
                        name: "/grafeas.v1beta1.project.Projects/DeleteProject".to_string(),
                        streaming: ::grpc::rt::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::rt::MethodHandlerUnary::new(move |o, p| handler_copy.delete_project(o, p))
                    },
                ),
            ],
        )
    }
}
