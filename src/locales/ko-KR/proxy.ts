/**
 * @description 프록시 관리 - 한국어
 */
export default {
  // 페이지 제목
  title: '프록시 관리',

  // 도구 모음
  toolbar: {
    addProxy: '프록시 추가',
    batchImport: '일괄 가져오기',
    batchExport: '일괄 내보내기',
    batchDelete: '일괄 삭제',
    batchCheck: '일괄 검사',
    filterByType: '유형별 필터',
    filterByStatus: '상태별 필터'
  },

  // 프록시 유형
  type: {
    http: 'HTTP',
    https: 'HTTPS',
    socks4: 'SOCKS4',
    socks5: 'SOCKS5',
    ssh: 'SSH'
  },

  // 프록시 상태
  status: {
    valid: '유효',
    invalid: '무효',
    checking: '검사 중',
    unchecked: '미검사',
    expired: '만료됨'
  },

  // 테이블 열
  column: {
    name: '프록시 이름',
    type: '유형',
    host: '호스트 주소',
    port: '포트',
    username: '사용자 이름',
    password: '비밀번호',
    status: '상태',
    delay: '지연',
    location: '위치',
    usedBy: '사용 수',
    expireTime: '만료 시간',
    lastCheck: '마지막 검사',
    createTime: '생성 시간',
    operation: '작업'
  },

  // 추가/편집 폼
  form: {
    title: '프록시 정보',
    addTitle: '프록시 추가',
    editTitle: '프록시 편집',
    name: '프록시 이름',
    namePlaceholder: '프록시 이름을 입력하세요',
    type: '프록시 유형',
    host: '호스트 주소',
    hostPlaceholder: '호스트 주소 또는 IP를 입력하세요',
    port: '포트 번호',
    portPlaceholder: '포트 번호를 입력하세요',
    username: '사용자 이름',
    usernamePlaceholder: '사용자 이름을 입력하세요 (선택사항)',
    password: '비밀번호',
    passwordPlaceholder: '비밀번호를 입력하세요 (선택사항)',
    remark: '비고',
    remarkPlaceholder: '비고를 입력하세요'
  },

  // 일괄 가져오기
  import: {
    title: '프록시 일괄 가져오기',
    description: '다음 형식으로 가져올 수 있습니다:',
    format1: 'host:port',
    format2: 'host:port:username:password',
    format3: 'type://host:port',
    format4: 'type://username:password@host:port',
    placeholder: '프록시 정보를 입력하세요 (한 줄에 하나씩)',
    parseResult: '분석 결과: 성공 {success}개, 실패 {failed}개',
    importButton: '가져오기 시작'
  },

  // 작업
  action: {
    check: '검사',
    edit: '편집',
    delete: '삭제',
    copyInfo: '정보 복사'
  },

  // 검사 결과
  checkResult: {
    success: '연결 성공',
    failed: '연결 실패',
    timeout: '연결 시간 초과',
    delay: '지연 {ms}ms',
    location: '위치: {location}'
  },

  // 빈 상태
  empty: {
    title: '프록시가 없습니다',
    description: '위의 버튼을 클릭하여 첫 번째 프록시를 추가하세요',
    addButton: '프록시 추가'
  },

  // 메시지
  message: {
    checkSuccess: '프록시 검사 성공, 지연 {ms}ms',
    checkFailed: '프록시 검사 실패: {reason}',
    deleteConfirm: '프록시 "{name}"을(를) 삭제하시겠습니까?',
    deleteWithProfiles: '이 프록시는 {count}개의 창에서 사용 중입니다. 삭제 후 이 창들은 직접 연결 모드로 전환됩니다. 삭제하시겠습니까?',
    importSuccess: '{count}개의 프록시를 가져왔습니다',
    exportSuccess: '{count}개의 프록시를 내보냈습니다'
  }
}
