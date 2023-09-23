const express = require('express');
const bodyParser = require('body-parser');

const app = express();
app.use(bodyParser.json());

// 用于存储用户的对象
const users = {};

// 注册路由
app.post('/register', (req, res) => {
    const { account, password } = req.body;
    if (!account || !password) {
        return res.status(400).json({
            status: 'error',
            code: '1001',
            message: 'Account and password are required.'
        });
    }
    
    if (users[account]) {
        return res.status(400).json({
            status: 'error',
            code: '1002',
            message: 'Account already exists.'
        });
    }
    
    users[account] = { password };
    return res.json({ status: 'success' });
});

// 登录路由
app.post('/login', (req, res) => {
    const { account, password } = req.body;
    if (!account || !password) {
        return res.status(400).json({
            status: 'error',
            code: '1001',
            message: 'Account and password are required.'
        });
    }
    
    const user = users[account];
    if (!user || user.password !== password) {
        return res.status(400).json({
            status: 'error',
            code: '2001',
            message: 'Wrong password or account.'
        });
    }
    
    return res.json({ status: 'success' });
});

// 启动服务
const port = 3000;
app.listen(port, () => console.log(`Server is running at http://localhost:${port}`));
