function main() {
    fetch("localhost:8080/health").then((e)=>{
        return e.json()
    }).then((e)=> console.log(e))

    fetch("localhost:8080/register/", {
        method: "POST",
        body: JSON.stringify({
            id: 1488421066,
            name: "Chocolate Tower",
            cuisine: "We serve tasty chocolate",
            address: "No 3 Don Linus Onwkaike Cresent",
            rating: 4,
            is_open: true,
        }),
        headers:{
            "content-type": "application/json"
        }
    }).then((e)=>{
        return e.json()
    }).then((e)=> console.log(e))

    fetch("localhost:8080/register/get").then((e)=>e.json()).then((e)=> console.log(e))
}


main()

function main2() {
    // ... existing health and register tests ...

    // Test adding menu items
    fetch("localhost:8080/menu/", {
        method: "POST",
        body: JSON.stringify([
            {
                id: 1,
                restaurant_id: 1488421066,
                name: "Chocolate Cake",
                description: "Rich chocolate cake with ganache",
                price: 12.99,
                category: "desserts",
                is_available: true
            },
            {
                id: 2,
                restaurant_id: 1488421066,
                name: "Hot Chocolate",
                description: "Creamy hot chocolate with marshmallows",
                price: 5.99,
                category: "drinks",
                is_available: true
            }
        ]),
        headers: {
            "content-type": "application/json"
        }
    }).then(e => e.json())
      .then(e => console.log("Add menu items response:", e));

    // Test getting all menu items
    fetch("localhost:8080/menu/get")
        .then(e => e.json())
        .then(e => console.log("All menu items:", e));

    // Test getting menu items by category
    fetch("localhost:8080/menu/get/drinks")
        .then(e => e.json())
        .then(e => console.log("Dessert items:", e));

    fetch("localhost:8080/menu/get/drinks")
        .then(e => e.json())
        .then(e => console.log("Drink items:", e));
}

main2();