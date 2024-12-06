# NYX
Assistant to help in daily development tasks.


## Application Data File 
The application data file is a JSON file that contains the data of the applications that NYX will manage. The file is located in the `data` folder and is named `app.json`. The file has the following structure:

```json
{
  "application": [
    {
      "id": "nyx",
      "name": "NYX",
      "tech": "Rust",
      "location": "/Users/elouan/Project/Personnal_project/NYX"
    },
    {
      "id": "tes",
      "name": "Test",
      "tech": "Golang",
      "location": "/Users/elouan/Project/Personnal_project/Test"
    },
    {
      "id": "mer",
      "name": "Mercure-Services",
      "tech": "Golang",
      "location": "/Users/elouan/Project/Work_project/orion/Mercure-Services"
    }
  ]
}
```