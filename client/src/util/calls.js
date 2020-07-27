const GET_PARAMS = {
  method: 'GET',
  headers: {
    'content-type': 'application/json',
  },
};

const GET_REQ = async (endpoint) => {
  fetch(API_URL+endpoint, GET_PARAMS)
}

export async function createRecord(recordData) {
  const post = await fetch(API_URL+'/user/record', {
    method: 'POST',
    headers: {
      'content-type': 'application/json',
    },
    body: JSON.stringify(recordData)
  })
      .catch(err=>{
          console.log(err);
      });
  if (post.ok) {
      return post.json();
  } else {
    throw new Error(users);
  }
}

export async function fetchAll() {
  const usrs = await fetch(API_URL+'/all', GET_PARAMS)
    .catch(err=>{
      console.log(err);
  });
  if (usrs.ok) {
    return usrs.json();
  } else {
    throw new Error(users);
  }
}

export async function getUserByUsername(username) {
    const usr = await fetch(API_URL+'/user/'+GET_PARAMS)
      .catch(err=>{
          console.log(err);
      });
  if (usr.ok) { 
    return usr.json();
  } else {
    throw new Error("No user with username " + username + " found.");
  }
}
export async function getUserById(id) {
    const usr = await fetch(API_URL+'/user/id/'+id, {
        method: 'GET',
        headers: {
          'content-type': 'application/json',
        },
    })
        .catch(err=>{
            console.log(err);
        });
  if (usr.ok) { 
    return usr.json();
  } else {
    throw new Error("No user with id " + id + " found.");
  }
}
