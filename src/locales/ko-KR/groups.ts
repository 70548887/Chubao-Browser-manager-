/**
 * @description 그룹 관리 - 한국어
 */
export default {
  // 페이지 제목
  title: '그룹 관리',

  // 도구 모음
  toolbar: {
    addGroup: '그룹 만들기',
    batchDelete: '일괄 삭제',
    sortBy: '정렬'
  },

  // 정렬 옵션
  sort: {
    lastUpdated: '최근 업데이트',
    name: '이름순',
    sortValue: '정렬 순서'
  },

  // 테이블 열
  column: {
    index: '번호',
    name: '그룹 이름',
    profileCount: '창 수',
    sort: '정렬 순서',
    remark: '비고',
    createTime: '생성 시간',
    operation: '작업'
  },

  // 추가/편집 폼
  form: {
    addTitle: '그룹 만들기',
    editTitle: '그룹 편집',
    name: '그룹 이름',
    namePlaceholder: '그룹 이름을 입력하세요',
    icon: '아이콘',
    remark: '비고',
    remarkPlaceholder: '비고를 입력하세요 (선택사항)',
    sort: '정렬 순서',
    sortTip: '값이 작을수록 위에 표시됩니다'
  },

  // 아이콘 옵션
  icon: {
    folder: '폴더',
    shopping: '쇼핑',
    campaign: '마케팅',
    movie: '동영상',
    payments: '결제',
    mail: '메일'
  },

  // 상세 드로어
  detail: {
    title: '그룹 상세',
    basicInfo: '기본 정보',
    statistics: '리소스 통계',
    relatedProfiles: '관련 창',
    activeSessions: '활성 세션',
    quickActions: '빠른 작업',
    batchProxy: '프록시 일괄 설정',
    manageExtensions: '확장 프로그램 관리',
    shareToTeam: '팀에 공유'
  },

  // 빈 상태
  empty: {
    title: '그룹이 없습니다',
    createButton: '그룹 만들기'
  },

  // 메시지
  message: {
    createSuccess: '그룹을 만들었습니다',
    updateSuccess: '그룹을 업데이트했습니다',
    deleteSuccess: '그룹을 삭제했습니다',
    deleteConfirm: '그룹 "{name}"을(를) 삭제하시겠습니까?',
    defaultGroupTip: '기본 그룹은 삭제할 수 없습니다',
    hasProfiles: '이 그룹에 {count}개의 창이 있습니다. 먼저 창을 이동하거나 삭제해 주세요',
    batchDeleteConfirm: '선택한 {count}개의 그룹을 삭제하시겠습니까? 이 작업은 되돌릴 수 없습니다!'
  },

  // 생성 힌트
  createTip: '그룹을 만든 후, 기존 브라우저 환경을 이 그룹으로 이동하거나 이 그룹에서 새 환경을 만들 수 있습니다.'
}
