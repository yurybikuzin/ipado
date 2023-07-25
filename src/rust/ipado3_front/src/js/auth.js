'use strict'

function auth(params) {
    const {APP, LOAD_APP} = params
    if (location.host.startsWith('local.') ){ 
        LOAD_APP(params)
    } else {
        import('https://z9v.ru/login/script.js?v=1')
            .then(m => {
                window.logout = m.logout
                window.login = m.login
                window.prepareLoginForm = m.prepareLoginForm
                m.auth({ on_complete })
            })
    }
    /* 
    # Код приложения после успешного прохождения пользователем процедуры аутентификации:

    ## ниже должно быть определение функции с двумя параметрами:
    - token: String
    - fingerprint: String
    */
    function on_complete(token, fingerprint) {
        /* 
        ## нужно сделать запрос к back'у приложения, чтобы он авторизовал аутентифицированного пользователя
        для авторизации back должен получить персональные данные пользователя следующим запросом
        curl -X POST https://z9v.ru/login/token \
         -H 'Content-type: application/json'\
         -d '{
            "token": "TOKEN",
            "fingerprint": "FINGERPRINT",
            "secret_key": "SECRET_KEY", 
         }'
        SECRET_KEY - это ключ доступа к сервису login

        ### в ответ на этот запрос back получает 
        - либо:
             { "err": "ERROR_DESCRIPTION" }
        - либо:
             { "ok": { "google": {
                "name": "NAME",
                "given_name": "GIVEN_NAME",
                "family_name": "FAMILY_NAME",
                "email": "EMAIL",
                "email_verified": true|false,
                "picture": "URL аватарки"
             }  } }
        - либо:
             { "ok": { "mailru": {
                "nickname": "NICKNAME",
                "email": "EMAIL",
                "name": "NAME",
                "first_name": "GIVEN_NAME",
                "last_name": "FAMILY_NAME",
                "birthday": "BIRTHDAY",
                "gender": "GENDER",
                "image": "URL аватарки"
             } } }
        - либо:
             { "ok": { "yandex": {
                "default_email": "DEFAULT_EMAIL",
                "emails": [ "EMAIL", ... ],
                "default_avatar_id": "DEFAULT_AVATAR_ID" // image = "https://avatars.mds.yandex.net/get-yapic/" + "DEFAULT_AVATAR_ID"
             } } }
        - либо:
             { "ok": { "tinkoff": {
                "PHONE_NUMBER_VERIFIED": true|false,
                "phone_number": "PHONE_NUMBER",
                "email_verified": true|false,
                "email": "EMAIL",
                "name": "NAME",
                "given_name": "GIVEN_NAME",
                "family_name": "FAMILY_NAME",
                "middle_name": "MIDDLE_NAME",
                "birthdate": "BIRTHDAY",
                "gender": "GENDER"
             } } }

          ### далее нужно найти в ACL (Access Control List) приложения полученные контакты и предоставить доступ пользователю с этими контактами
        */
        const url = location.protocol + '//' + location.host + 
            (
                location.host.startsWith('local') ?  '' :
                location.pathname.startsWith('/dev/') ? '/dev' :
                location.pathname.startsWith('/demo/') ? '/demo' :
                location.pathname.startsWith('/rc/') ? '/rc' :
                ''
            ) + '/' + APP + '_back/login'
        fetch(url, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ token, fingerprint })
        })
            .then(resp => resp.json())
            .then(resp => { 
                if (resp.err) {
                    // ### в случае, если back отказал пользователю в авторизации, 
                    // то возвращаем его к началу процесса login
                    localStorage.removeItem('loginToken')
                    const message = resp.err.email ? resp.err.email : resp.err.phoneNumber ? resp.err.phoneNumber : ' пользователя с неизвестными контактами'
                    const el = document.getElementById('no_access')
                    if (el) {
                        el.innerText = message
                    } else {
                        prepareLoginForm({on_complete}).then(() => {
                            const el = document.getElementById('no_access')
                            if (el) {
                                el.innerText = message
                            } else {
                                console.error(resp.err)
                            }
                        })
                    }
                } else {
                    // ### в случае. если back авторизовал пользователя
                    // то пропускаем пользователя в приложение
                    LOAD_APP(params)
                }
            })
    }
}

export { auth }
