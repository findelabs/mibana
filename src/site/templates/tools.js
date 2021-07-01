$('#collections').change(function(){
      var url = "/query?";
      
      if($("#collection").val()!='Select')
        url+='collection='+encodeURIComponent($("#collections").val())+'&';
      
      url = url.replace(/\&$/,'');
      window.location.href=url;
});

$('#search_field').on('submit',function(e){
    e.preventDefault();
    var formData=$(this).serialize();
    var url = [location.protocol, '//', location.host, location.pathname].join('');
    var collection = urlParams.get('collection')
    var finalUrl = url+"?collection="+collection+"&"+formData;
    window.location.href = finalUrl;
}) 

/*
$('.link').click(function () {
  $(this).toggle(function(){
    $(this).animate({height: 1000});
  },function(){
    $(this).animate({height: 200});
  });
});


$('.link').click(function () {
  if ($(this).height() == 200) {
    $(this).animate("height",1000);
  }
  else if ($(this).height() == 1000) {
    $(this).animate("height",200);
  });
});
*/

$(document).ready(function (){
    $(".link").on("click", function (){
        if ($(this).height() == 200) {
            var el = $(this),
                curHeight = el.height(),
                autoHeight = el.css('height', 'auto').height();

            el.height(curHeight).animate(
                {height: autoHeight});
            }
        else if ($(this).height() != 200) {
            $(this).animate({height: "200px"});
            }
        });
    });
