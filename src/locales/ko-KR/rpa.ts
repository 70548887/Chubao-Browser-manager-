/**
 * @description RPA 로봇 - 한국어
 */
export default {
  // 페이지 제목
  title: 'RPA 로봇',

  // 도구 모음
  toolbar: {
    createFlow: '플로우 만들기',
    importFlow: '플로우 가져오기',
    filterByStatus: '상태별 필터'
  },

  // 플로우 상태
  status: {
    draft: '초안',
    ready: '준비됨',
    running: '실행 중',
    paused: '일시 중지',
    completed: '완료',
    failed: '실패',
    scheduled: '예약됨'
  },

  // 테이블 열
  column: {
    name: '플로우 이름',
    description: '설명',
    status: '상태',
    lastRun: '마지막 실행',
    nextRun: '다음 실행',
    runCount: '실행 횟수',
    successRate: '성공률',
    createTime: '생성 시간',
    operation: '작업'
  },

  // 작업
  action: {
    run: '실행',
    pause: '일시 중지',
    stop: '중지',
    edit: '편집',
    clone: '복제',
    delete: '삭제',
    viewLogs: '로그 보기',
    schedule: '예약'
  },

  // 플로우 편집기
  editor: {
    title: '플로우 편집기',
    save: '플로우 저장',
    run: '플로우 실행',
    undo: '실행 취소',
    redo: '다시 실행',
    zoomIn: '확대',
    zoomOut: '축소',
    fitView: '뷰에 맞추기',
    nodeLibrary: '노드 라이브러리',
    properties: '속성 패널'
  },

  // 노드 유형
  node: {
    browser: '브라우저 작업',
    navigation: '페이지 이동',
    click: '클릭',
    input: '텍스트 입력',
    extract: '데이터 추출',
    condition: '조건 분기',
    loop: '반복',
    delay: '지연',
    screenshot: '스크린샷',
    script: '사용자 정의 스크립트'
  },

  // 빈 상태
  empty: {
    title: '플로우가 없습니다',
    description: '첫 번째 자동화 플로우를 만들어 작업 효율을 높이세요',
    createButton: '플로우 만들기'
  },

  // 메시지
  message: {
    createSuccess: '플로우를 만들었습니다',
    saveSuccess: '플로우를 저장했습니다',
    runSuccess: '플로우 실행을 시작했습니다',
    stopSuccess: '플로우를 중지했습니다',
    deleteConfirm: '플로우 "{name}"을(를) 삭제하시겠습니까?'
  }
}
