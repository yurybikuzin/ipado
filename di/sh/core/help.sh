cat << EOM
~/$(realpath --releative-base ~ "$0") (version $VERSION) is core of Deploy Infrastructure
This file is to be symlinked in di/\$kind/\$app 
    (where \$kind is one of 'dev' or 'prod', \$app is an application of project)
by following command:
    $0 -l
Common usage is (from project root):
    di/\$kind/\$app/nginx_conf.sh
    di/\$kind/\$app/deploy.sh
EOM
exit 0
