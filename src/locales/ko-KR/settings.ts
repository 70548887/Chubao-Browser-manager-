/**
 * @description 설정 - 한국어
 */
export default {
  // 페이지 제목
  title: '설정',

  // 설정 카테고리
  category: {
    general: '일반 설정',
    appearance: '외관',
    language: '언어',
    browser: '브라우저',
    proxy: '프록시',
    storage: '저장소',
    notification: '알림',
    security: '보안',
    about: '정보'
  },

  // 일반 설정
  general: {
    title: '일반 설정',
    startOnBoot: '시스템 시작 시 자동 실행',
    startOnBootDesc: '시스템 시작 시 앱을 자동으로 실행',
    minimizeToTray: '트레이로 최소화',
    minimizeToTrayDesc: '창을 닫으면 시스템 트레이로 최소화',
    checkUpdate: '자동 업데이트 확인',
    checkUpdateDesc: '시작 시 새 버전 확인',
    analytics: '사용 통계',
    analyticsDesc: '제품 개선을 위해 익명 사용 데이터 전송'
  },

  // 외관 설정
  appearance: {
    title: '외관 설정',
    theme: '테마',
    themeLight: '라이트',
    themeDark: '다크',
    themeAuto: '시스템 설정 따름',
    accentColor: '강조 색상',
    fontSize: '글꼴 크기',
    fontSizeSmall: '작게',
    fontSizeNormal: '보통',
    fontSizeLarge: '크게',
    sidebarWidth: '사이드바 너비',
    compactMode: '컴팩트 모드',
    compactModeDesc: 'UI 요소 간격 줄이기'
  },

  // 언어 설정
  language: {
    title: '언어 설정',
    displayLanguage: '표시 언어',
    selectLanguage: '언어 선택',
    languageZhCN: '简体中文',
    languageEnUS: 'English',
    languageJaJP: '日本語',
    languageKoKR: '한국어',
    restartRequired: '언어 변경 후 앱 재시작이 필요합니다'
  },

  // 브라우저 설정
  browser: {
    title: '브라우저 설정',
    kernelPath: '커널 경로',
    kernelPathDesc: '브라우저 커널이 있는 디렉토리',
    selectPath: '경로 선택',
    defaultProfile: '기본 설정',
    cacheSize: '캐시 크기 제한',
    clearCache: '캐시 지우기',
    clearCacheDesc: '모든 브라우저 캐시 데이터 삭제'
  },

  // 프록시 설정
  proxy: {
    title: '프록시 설정',
    globalProxy: '전역 프록시',
    globalProxyDesc: '모든 창에 기본 프록시 설정',
    noProxy: '프록시 없음',
    selectProxy: '프록시 선택',
    timeout: '연결 시간 초과',
    timeoutDesc: '프록시 연결 시간 초과 (초)',
    retryCount: '재시도 횟수',
    retryCountDesc: '연결 실패 시 재시도 횟수'
  },

  // 저장소 설정
  storage: {
    title: '저장소 설정',
    dataPath: '데이터 저장 경로',
    dataPathDesc: '창 설정 및 데이터 저장 위치',
    usedSpace: '사용 공간',
    totalSpace: '전체 공간',
    cleanUp: '정리',
    cleanUpDesc: '임시 파일 및 불필요한 데이터 삭제',
    export: '데이터 내보내기',
    exportDesc: '모든 창 설정 및 설정 내보내기',
    import: '데이터 가져오기',
    importDesc: '백업 파일에서 데이터 복원'
  },

  // 알림 설정
  notification: {
    title: '알림 설정',
    enableNotification: '알림 활성화',
    enableNotificationDesc: '시스템 알림 표시',
    sound: '알림 소리',
    soundDesc: '알림 소리 재생',
    taskComplete: '작업 완료 알림',
    taskCompleteDesc: 'RPA 작업 완료 시 알림',
    updateAvailable: '업데이트 알림',
    updateAvailableDesc: '새 버전이 있을 때 알림'
  },

  // 보안 설정
  security: {
    title: '보안 설정',
    password: '접근 비밀번호',
    passwordDesc: '앱 시작 비밀번호 설정',
    setPassword: '비밀번호 설정',
    changePassword: '비밀번호 변경',
    removePassword: '비밀번호 제거',
    autoLock: '자동 잠금',
    autoLockDesc: '일정 시간 유휴 시 자동 잠금',
    lockTime: '잠금 시간',
    encryption: '데이터 암호화',
    encryptionDesc: '민감한 데이터 암호화 저장'
  },

  // 정보
  about: {
    title: '정보',
    appName: '브라우저 멀티오프너',
    version: '버전',
    buildTime: '빌드 시간',
    copyright: '저작권 정보',
    website: '공식 웹사이트',
    documentation: '문서',
    feedback: '피드백',
    checkUpdate: '업데이트 확인',
    currentVersion: '현재 버전이 최신입니다',
    newVersion: '새 버전 {version}을(를) 사용할 수 있습니다',
    changelog: '업데이트 로그'
  },

  // 메시지
  message: {
    saveSuccess: '설정을 저장했습니다',
    restartRequired: '일부 설정은 앱 재시작 후 적용됩니다',
    exportSuccess: '데이터를 내보냈습니다',
    importSuccess: '데이터를 가져왔습니다',
    cleanUpSuccess: '정리 완료, {size} 해제됨',
    passwordSet: '비밀번호를 설정했습니다',
    passwordChanged: '비밀번호를 변경했습니다',
    passwordRemoved: '비밀번호를 제거했습니다'
  }
}
