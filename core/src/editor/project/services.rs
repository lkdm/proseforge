

///
/// Service contains functions that more directly relate to the business logic of the application.
/// A service function may call multiple repository functions to accomplish its task.
///
/// It may also publish events or perform other side effects.
///
#[derive(Debug, Clone)]
pub struct DesktopService<R>
where
    R: ProjectRepository,
    F: ProjectRepository,
    // M: AuthorMetrics,
    // N: AuthorNotifier,
{
    repo: R,
    // metrics: M,
    // notifier: N,
}

impl<R> DesktopService<R>
where
    R: ProjectRepository,
    // M: SomeMetricsTrait,
    // N: SomeNotifierTrait,
{
    pub fn new(repo: R) -> Self {
        Service { repo }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CreateProjectRequestDto {}

pub trait DocumentService {
    fn project_create_db(
        &self,
        req: &CreateDocumentRequestDto,
    ) -> impl Future<Output = Result<Id, ServiceError>> + Send;
    fn document_update(
        &self,
        req: &UpdateDocumentRequestDto,
    ) -> impl Future<Output = Result<(), ServiceError>> + Send;
    fn document_get(
        &self,
        req: &GetDocumentRequestDto,
    ) -> impl Future<Output = Result<GetDocumentResponseDto, ServiceError>> + Send;
}
