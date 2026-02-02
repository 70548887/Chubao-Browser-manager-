/**
 * @description 창 목록/대시보드 - 한국어
 */
export default {
  // 페이지 제목
  title: '창 목록',

  // 도구 모음
  toolbar: {
    createWindow: '창 만들기',
    batchOpen: '일괄 열기',
    batchClose: '일괄 닫기',
    batchDelete: '일괄 삭제',
    importProfile: '설정 가져오기',
    exportProfile: '설정 내보내기',
    sortBy: '정렬',
    filterBy: '필터',
    viewMode: '보기 모드',
    listView: '목록 보기',
    gridView: '그리드 보기'
  },

  // 정렬 옵션
  sort: {
    lastUpdated: '최근 업데이트',
    name: '이름',
    createTime: '생성 시간',
    lastOpened: '최근 열림'
  },

  // 필터 옵션
  filter: {
    all: '전체',
    running: '실행 중',
    stopped: '중지됨',
    byGroup: '그룹별',
    byTag: '태그별',
    byProxy: '프록시별'
  },

  // 창 카드
  card: {
    id: 'ID',
    group: '그룹',
    proxy: '프록시',
    tags: '태그',
    lastOpened: '마지막 열림',
    createTime: '생성 시간',
    noProxy: '직접 연결',
    noGroup: '미분류',
    noTags: '태그 없음'
  },

  // 창 작업
  action: {
    open: '창 열기',
    close: '창 닫기',
    edit: '설정 편집',
    clone: '창 복제',
    delete: '창 삭제',
    moveToGroup: '그룹으로 이동',
    addTags: '태그 추가',
    setProxy: '프록시 설정',
    viewDetail: '상세 보기',
    copyId: 'ID 복사'
  },

  // 창 생성 마법사
  createWizard: {
    title: '창 만들기',
    step1: '기본 설정',
    step2: '프록시 설정',
    step3: '핑거프린트 설정',
    step4: '환경 설정',
    basicInfo: '기본 정보',
    windowName: '창 이름',
    windowNamePlaceholder: '창 이름을 입력하세요',
    selectGroup: '그룹 선택',
    selectTags: '태그 선택',
    remark: '비고',
    remarkPlaceholder: '비고를 입력하세요 (선택사항)'
  },

  // 프록시 설정
  proxySettings: {
    title: '프록시 설정',
    proxyType: '프록시 유형',
    directConnect: '직접 연결 (프록시 없음)',
    selectProxy: '기존 프록시 선택',
    createProxy: '새 프록시 만들기',
    noProxy: '프록시가 없습니다. 먼저 만들어 주세요'
  },

  // 핑거프린트 설정
  fingerprintSettings: {
    title: '핑거프린트 설정',
    autoGenerate: '자동 생성',
    manualConfig: '수동 설정',
    browserCore: '브라우저 코어',
    userAgent: 'User Agent',
    screenResolution: '화면 해상도',
    language: '언어',
    timezone: '시간대',
    webGL: 'WebGL',
    canvas: 'Canvas',
    audio: '오디오 핑거프린트',
    fonts: '글꼴 목록',
    plugins: '플러그인 목록'
  },

  // 환경 설정
  preferenceSettings: {
    title: '환경 설정',
    startUrl: '시작 페이지',
    startUrlPlaceholder: '시작 시 열릴 URL을 입력하세요',
    downloadPath: '다운로드 경로',
    cookieSync: 'Cookie 동기화',
    localStorage: 'LocalStorage',
    cache: '캐시 정책'
  },

  // 빈 상태
  empty: {
    title: '창이 없습니다',
    description: '위의 버튼을 클릭하여 첫 번째 브라우저 창을 만드세요',
    createButton: '창 만들기'
  },

  // 일괄 작업
  batch: {
    selected: '{count}개의 창 선택됨',
    confirmDelete: '선택한 {count}개의 창을 삭제하시겠습니까? 이 작업은 되돌릴 수 없습니다!',
    confirmClose: '선택한 {count}개의 실행 중인 창을 닫으시겠습니까?',
    openSuccess: '{count}개의 창을 열었습니다',
    closeSuccess: '{count}개의 창을 닫았습니다',
    deleteSuccess: '{count}개의 창을 삭제했습니다'
  }
}
