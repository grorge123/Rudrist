import requests

base_url = "http://localhost:3000"  

def test_register(account, password):
    response = requests.post(f"{base_url}/register", json={"account": account, "password": password})
    print(f"Register {account}: {response.status_code}, {response.json()}")
    return response

def test_login(account, password):
    response = requests.post(f"{base_url}/login", json={"account": account, "password": password})
    print(f"Login {account}: {response.status_code}, {response.json()}")
    return response

def main():
    account = "test_account"
    password = "test_password"
    
    response = test_register(account, password)
    assert response.status_code == 200
    assert response.json()["status"] == "success"
    
    response = test_login(account, password)
    assert response.status_code == 200
    assert response.json()["status"] == "success"
    
    response = test_login("nonexistent_account", "some_password")
    assert response.status_code == 400
    assert response.json()["status"] == "error"
    
if __name__ == "__main__":
    main()
