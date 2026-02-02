/**
 * @description 공통 용어 - 한국어
 */
export default {
  // 액션 버튼
  action: {
    add: '새로 만들기',
    create: '생성',
    edit: '편집',
    delete: '삭제',
    remove: '제거',
    save: '저장',
    cancel: '취소',
    confirm: '확인',
    close: '닫기',
    search: '검색',
    filter: '필터',
    sort: '정렬',
    refresh: '새로고침',
    reset: '초기화',
    export: '내보내기',
    import: '가져오기',
    copy: '복사',
    paste: '붙여넣기',
    selectAll: '모두 선택',
    batchDelete: '일괄 삭제',
    batchOpen: '일괄 열기',
    batchClose: '일괄 닫기',
    start: '시작',
    stop: '중지',
    open: '열기',
    view: '보기',
    detail: '상세',
    back: '뒤로',
    next: '다음',
    prev: '이전',
    submit: '제출',
    apply: '적용',
    test: '테스트',
    check: '검사',
    retry: '재시도',
    more: '더보기',
    expand: '펼치기',
    collapse: '접기',
    download: '다운로드',
    upload: '업로드',
    install: '설치',
    uninstall: '제거',
    enable: '활성화',
    disable: '비활성화',
    restore: '복원',
    clear: '지우기'
  },

  // 상태
  status: {
    running: '실행 중',
    stopped: '중지됨',
    starting: '시작 중',
    stopping: '중지 중',
    online: '온라인',
    offline: '오프라인',
    connected: '연결됨',
    disconnected: '연결 끊김',
    enabled: '활성화됨',
    disabled: '비활성화됨',
    active: '활성',
    inactive: '비활성',
    pending: '대기 중',
    processing: '처리 중',
    completed: '완료',
    failed: '실패',
    error: '오류',
    warning: '경고',
    success: '성공',
    loading: '로딩 중',
    saving: '저장 중',
    deleting: '삭제 중',
    unknown: '알 수 없음',
    valid: '유효',
    invalid: '무효',
    expired: '만료됨'
  },

  // 메시지
  message: {
    confirmDelete: '삭제하시겠습니까? 이 작업은 되돌릴 수 없습니다!',
    confirmBatchDelete: '선택한 {count}개 항목을 삭제하시겠습니까? 이 작업은 되돌릴 수 없습니다!',
    deleteSuccess: '삭제되었습니다',
    deleteFailed: '삭제 실패',
    saveSuccess: '저장되었습니다',
    saveFailed: '저장 실패',
    createSuccess: '생성되었습니다',
    createFailed: '생성 실패',
    updateSuccess: '업데이트되었습니다',
    updateFailed: '업데이트 실패',
    copySuccess: '클립보드에 복사되었습니다',
    copyFailed: '복사 실패',
    operationSuccess: '작업이 완료되었습니다',
    operationFailed: '작업 실패',
    loadingData: '데이터를 불러오는 중...',
    noData: '데이터가 없습니다',
    noMoreData: '더 이상 데이터가 없습니다',
    networkError: '네트워크 오류입니다. 연결을 확인해 주세요',
    serverError: '서버 오류입니다. 나중에 다시 시도해 주세요',
    unknownError: '알 수 없는 오류',
    inputRequired: '{field}을(를) 입력해 주세요',
    selectRequired: '{field}을(를) 선택해 주세요',
    invalidFormat: '{field} 형식이 올바르지 않습니다',
    lengthLimit: '{field}은(는) {max}자를 초과할 수 없습니다',
    confirmUnsaved: '저장되지 않은 변경사항이 있습니다. 정말 나가시겠습니까?'
  },

  // 폼 레이블
  label: {
    name: '이름',
    title: '제목',
    description: '설명',
    remark: '비고',
    type: '유형',
    status: '상태',
    sort: '정렬',
    createTime: '생성 시간',
    updateTime: '수정 시간',
    deleteTime: '삭제 시간',
    creator: '생성자',
    operator: '작업자',
    operation: '작업',
    enable: '활성화',
    required: '필수',
    optional: '선택'
  },

  // 페이지네이션
  pagination: {
    total: '전체 {total}건',
    pageSize: '{size}건/페이지',
    goto: '이동',
    page: '페이지',
    showing: '{from}~{to}건 표시, 전체 {total}건'
  },

  // 시간
  time: {
    today: '오늘',
    yesterday: '어제',
    lastWeek: '지난주',
    lastMonth: '지난달',
    justNow: '방금',
    minutesAgo: '{n}분 전',
    hoursAgo: '{n}시간 전',
    daysAgo: '{n}일 전'
  },

  // 단위
  unit: {
    item: '개',
    piece: '건',
    times: '회',
    second: '초',
    minute: '분',
    hour: '시간',
    day: '일',
    week: '주',
    month: '월',
    year: '년'
  },

  // 확인 다이얼로그
  dialog: {
    confirmTitle: '확인',
    warningTitle: '경고',
    errorTitle: '오류',
    infoTitle: '알림',
    yes: '예',
    no: '아니오',
    ok: '확인',
    cancel: '취소'
  },

  // 빈 상태
  empty: {
    default: '데이터가 없습니다',
    search: '검색 결과가 없습니다',
    filter: '필터 조건에 맞는 데이터가 없습니다'
  }
}
