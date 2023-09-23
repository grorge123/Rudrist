package main

import (
	"sync"

	"github.com/gin-gonic/gin"
)

type Account struct {
	Username string `json:"account" binding:"required"`
	Password string `json:"password" binding:"required"`
}

var (
	mu       sync.RWMutex
	accounts = make(map[string]string)
)

func main() {
	r := gin.Default()
	r.POST("/register", register)
	r.POST("/login", login)
	r.Run(":3000") // 在 0.0.0.0:3000 上监听并在此端口上启动服务
}

func register(c *gin.Context) {
	var account Account
	if err := c.ShouldBindJSON(&account); err != nil {
		c.JSON(400, gin.H{"status": "error", "code": "1001", "message": "Account and password are required."})
		return
	}
	mu.Lock()
	defer mu.Unlock()
	if _, exists := accounts[account.Username]; exists {
		c.JSON(400, gin.H{"status": "error", "code": "1002", "message": "Account already exists."})
		return
	}
	accounts[account.Username] = account.Password
	c.JSON(200, gin.H{"status": "success"})
}

func login(c *gin.Context) {
	var account Account
	if err := c.ShouldBindJSON(&account); err != nil {
		c.JSON(400, gin.H{"status": "error", "code": "1001", "message": "Account and password are required."})
		return
	}
	mu.RLock()
	defer mu.RUnlock()
	if password, exists := accounts[account.Username]; exists && password == account.Password {
		c.JSON(200, gin.H{"status": "success"})
		return
	}
	c.JSON(400, gin.H{"status": "error", "code": "2001", "message": "Wrong password or account."})
}
