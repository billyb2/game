/*
lz4.js - LZ4 for browser

The MIT License (MIT)

Copyright (c) 2013, Syu Kato <ukyo.web@gmail.com>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.



LZ4 - Fast LZ compression algorithm
Copyright (C) 2011-2013, Yann Collet.
BSD 2-Clause License (http://www.opensource.org/licenses/bsd-license.php)

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are
met:

   * Redistributions of source code must retain the above copyright
notice, this list of conditions and the following disclaimer.
   * Redistributions in binary form must reproduce the above
copyright notice, this list of conditions and the following disclaimer
in the documentation and/or other materials provided with the
distribution.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
"AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
(INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
*/

var lz4init=function(){var xf="undefined"!=typeof document&&document.currentScript?document.currentScript.src:void 0;return"undefined"!=typeof __filename&&(xf=xf||__filename),function(f){var i,e,a;f=f||{},(i=i||(void 0!==f?f:{})).ready=new Promise(function(f,r){e=f,a=r});var r,b={};for(r in i)i.hasOwnProperty(r)&&(b[r]=i[r]);function k(f,r){throw r}var o,t,n,s,u=!1,c=!1,l=!1,u="object"==typeof window,c="function"==typeof importScripts,A="";(l="object"==typeof process&&"object"==typeof process.versions&&"string"==typeof process.versions.node)?(A=c?require("path").dirname(A)+"/":__dirname+"/",o=function(f,r){var e=P(f);return e?r?e:e.toString():(n=n||require("fs"),f=(s=s||require("path")).normalize(f),n.readFileSync(f,r?null:"utf8"))},t=function(f){return(f=o(f,!0)).buffer||(f=new Uint8Array(f)),f.buffer||O("Assertion failed: undefined"),f},1<process.argv.length&&process.argv[1].replace(/\\/g,"/"),process.argv.slice(2),process.on("uncaughtException",function(f){if(!(f instanceof af))throw f}),process.on("unhandledRejection",O),k=function(f){process.exit(f)},i.inspect=function(){return"[Emscripten Module object]"}):(u||c)&&(c?A=self.location.href:"undefined"!=typeof document&&document.currentScript&&(A=document.currentScript.src),xf&&(A=xf),A=0!==A.indexOf("blob:")?A.substr(0,A.lastIndexOf("/")+1):"",o=function(r){try{var e=new XMLHttpRequest;return e.open("GET",r,!1),e.send(null),e.responseText}catch(f){if(r=P(r)){e=[];for(var i=0;i<r.length;i++){var a=r[i];255<a&&(X&&O("Assertion failed: Character code "+a+" ("+String.fromCharCode(a)+")  at offset "+i+" not in 0x00-0xFF."),a&=255),e.push(String.fromCharCode(a))}return e.join("")}throw f}},c&&(t=function(r){try{var f=new XMLHttpRequest;return f.open("GET",r,!1),f.responseType="arraybuffer",f.send(null),new Uint8Array(f.response)}catch(f){if(r=P(r))return r;throw f}})),i.print||console.log.bind(console);var h=i.printErr||console.warn.bind(console);for(r in b)b.hasOwnProperty(r)&&(i[r]=b[r]);b=null,i.quit&&(k=i.quit),i.wasmBinary&&(v=i.wasmBinary);var p=i.noExitRuntime||!0;function m(){}function y(){this.exports=function(f){for(var e,n=new Uint8Array(123),r=25;0<=r;--r)n[48+r]=52+r,n[65+r]=r,n[97+r]=26+r;function M(f,r,e){for(var i,a,b=0,k=r,o=e.length,t=r+(3*o>>2)-("="==e[o-2])-("="==e[o-1]);b<o;b+=4)i=n[e.charCodeAt(b+1)],a=n[e.charCodeAt(b+2)],f[k++]=n[e.charCodeAt(b)]<<2|i>>4,k<t&&(f[k++]=i<<4|a>>2),k<t&&(f[k++]=a<<6|n[e.charCodeAt(b+3)])}return n[43]=62,n[47]=63,function(f){var i=f.a.buffer,df=new Int8Array(i),vf=new Int16Array(i),wf=new Int32Array(i),gf=new Uint8Array(i),Sf=new Uint16Array(i),g=new Uint32Array(i),Bf=(new Float32Array(i),new Float64Array(i),Math.imul),r=(Math.fround,Math.abs,Math.clz32),l=(Math.min,Math.max,Math.floor,Math.ceil,Math.trunc,Math.sqrt,f.abort),o=f.b,k=f.c,a=f.d,_f=5245424,u=0;function A(f,r,e,i,a,b,k){var o,t,n=0,s=0,u=0,c=0,l=0,A=0,h=0,p=0,m=0,y=0,d=0,v=0,w=0,g=0,S=0,B=0,_=0,Z=0,E=0,I=0,C=0,R=0,U=0,J=0,F=0,M=0,x=0,O=0,V=0,W=0,L=0,j=0,T=0,N=0,X=0,D=0,P=0,Y=0,z=0,G=0,Q=0,H=0,q=0,K=0,$=0,ff=0,rf=0,ef=0,af=0,bf=0,kf=0,of=0,tf=0,nf=0,sf=0,uf=0,cf=0,lf=0,Af=0,hf=0,pf=0,mf=0,yf=0;_f=Z=_f-65600|0;f:if(65536<=(n=((u=wf[f+262144>>2])-(B=wf[f+262148>>2])|0)-wf[f+262160>>2]|0)>>>0)wf[f+262172>>2]=0,n=Zf(f,r,e,i,a,b,k);else if(wf[i>>2]<4097|n){if(n=0,!(2==(0|k)&&(0|a)<=0||2113929216<(s=wf[i>>2])>>>0)){wf[f+262144>>2]=s+u,b=Bf(n=(0|(u=(0|b)<1?9:b))<12?u:12,12),t=wf[b+1764>>2];r:{e:if(n>>>0<=9){mf=e+a|(wf[i>>2]=0),o=(yf=2==(0|k))?mf-5|0:mf,lf=r+s|0,m=r,y=e;i:if(!((0|s)<13||(Af=lf-12|0)>>>0<r>>>0))for(pf=6656>>>n&1,af=(_=lf-5|0)-1|0,D=_-3|0,kf=g=L=x=Y=hf=3|Z,q=f+131072|0,m=S=r;;){if(a=wf[f+262160>>2],K=S-B|0,T=K>>>0<a+65536>>>0?a:K-65535|0,O=wf[f+262156>>2],d=gf[0|S]|gf[S+1|0]<<8|(gf[S+2|0]<<16|gf[S+3|0]<<24),cf=wf[f+262152>>2],uf=wf[f+262172>>2],n=wf[f+262164>>2],!(K>>>0<=n>>>0)&&(u=(-1^n)+S|0,(S-n|0)-B&1&&(b=(Bf(gf[0|(a=n+B|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24),-1640531535)>>>15&131068)+f|0,a=n-wf[b>>2]|0,vf[131072+(((65535&n)<<1)+f|0)>>1]=a>>>0<65535?a:65535,n=(wf[b>>2]=n)+1|0),(0|u)!=(0|B)))for(;b=(Bf(gf[0|(a=n+B|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24),-1640531535)>>>15&131068)+f|0,a=n-wf[b>>2]|0,vf[q+((65535&n)<<1)>>1]=a>>>0<65535?a:65535,wf[b>>2]=n,b=(Bf(gf[0|(a=(u=n+1|0)+B|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24),-1640531535)>>>15&131068)+f|0,a=u-wf[b>>2]|0,vf[q+((65535&u)<<1)>>1]=a>>>0<65535?a:65535,wf[b>>2]=u,(n=n+2|0)>>>0<K>>>0;);wf[f+262164>>2]=K,J=S+8|0,v=S+4|0,u=3;a:if((l=wf[(Bf(gf[0|S]|gf[S+1|0]<<8|(gf[S+2|0]<<16|gf[S+3|0]<<24),-1640531535)>>>15&131068)+f>>2])>>>0<T>>>0)s=t;else for(Q=(65535&d)==(d>>>16|0)&(0|(tf=d>>>24|0))==(255&d),V=O+cf|0,rf=(h=B+O|0)+4|0,N=S-1|0,of=4-v|0,s=t,c=I=0;;){b:{k:{o:{t:{n:{if(O>>>0<=l>>>0){if((0|(b=gf[0|(a=u+N|0)]|gf[a+1|0]<<8))!=(gf[0|(a=((A=l+B|0)+u|0)-1|0)]|gf[a+1|0]<<8)|(0|d)!=(gf[0|A]|gf[A+1|0]<<8|(gf[A+2|0]<<16|gf[A+3|0]<<24)))break b;if(n=A+4|0,(a=D)>>>0<=v>>>0)b=v;else{if(b=(gf[0|v]|gf[v+1|0]<<8|(gf[v+2|0]<<16|gf[v+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24)))break n;n=n+4|0,b=J}if(b>>>0<a>>>0)for(;;){if(a=(gf[0|b]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))){n=((Rf(a)>>>3|0)+b|0)-v|0;break k}if(n=n+4|0,!((b=b+4|0)>>>0<D>>>0))break}(gf[0|n]|gf[n+1|0]<<8)!=(gf[0|b]|gf[b+1|0]<<8)|af>>>0<=b>>>0||(n=n+2|0,b=b+2|0),b>>>0<_>>>0&&(b=gf[0|n]==gf[0|b]?b+1|0:b),n=b-v|0;break k}if((0|d)!=(gf[0|(a=l+cf|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24)))break b;if(n=a+4|0,!((E=(a=_>>>0<(C=(O-l|0)+S|0)>>>0?_:C)-3|0)>>>0<=(A=b=v)>>>0)){if(b=(gf[0|v]|gf[v+1|0]<<8|(gf[v+2|0]<<16|gf[v+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24)))break t;n=n+4|0,A=J}if((b=A)>>>0<E>>>0)for(;;){if(A=(gf[0|b]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))){b=((Rf(A)>>>3|0)+b|0)-v|0;break o}if(n=n+4|0,!((b=b+4|0)>>>0<E>>>0))break}(gf[0|n]|gf[n+1|0]<<8)!=(gf[0|b]|gf[b+1|0]<<8)|a-1>>>0<=b>>>0||(n=n+2|0,b=b+2|0),b>>>0<a>>>0&&(b=gf[0|n]==gf[0|b]?b+1|0:b),b=b-v|0;break o}n=Rf(b)>>>3|0;break k}b=Rf(b)>>>3|0}if(F=l+B|0,!((0|a)!=(S+(A=b+4|0)|0)|_>>>0<=C>>>0)){b=h;o:{t:{if((n=a)>>>0<D>>>0){if(b=(gf[0|a]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24))^(gf[0|h]|gf[h+1|0]<<8|(gf[h+2|0]<<16|gf[h+3|0]<<24)))break t;n=a+4|0,b=rf}if(n>>>0<D>>>0)for(;;){if(E=(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))^(gf[0|b]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24))){a=((Rf(E)>>>3|0)+n|0)-a|0;break o}if(b=b+4|0,!((n=n+4|0)>>>0<D>>>0))break}(gf[0|b]|gf[b+1|0]<<8)!=(gf[0|n]|gf[n+1|0]<<8)|af>>>0<=n>>>0||(n=n+2|0,b=b+2|0),n>>>0<_>>>0&&(n=gf[0|b]==gf[0|n]?n+1|0:n),a=n-a|0;break o}a=Rf(b)>>>3|0}A=a+A|0}R=(a=(0|u)<(0|A))?F:R,u=a?A:u;break b}u=(b=(0|u)<(0|(a=n+4|0)))?a:u,R=b?A:R}s=s-1|0;b:{k:{o:{t:if(!(!pf|1!=(0|(A=Sf[131072+(((65535&l)<<1)+f|0)>>1])))){if(!I){if(I=1,!Q)break t;n:{s:if(!(D>>>0<=(n=v)>>>0)){for(;!(a=d^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24)));)if(!((n=n+4|0)>>>0<D>>>0))break s;n=(Rf(a)>>>3|0)+n|0;break n}if(b=d,!(_>>>0<=n>>>0)){for(;;){if(gf[0|n]!=(255&b))break n;if(b=b>>>8|0,(0|_)==(0|(n=n+1|0)))break}n=_}}I=2,c=n+of|0}if(z=l-1|0,!(2!=(0|I)|z>>>0<T>>>0||(I=2,O-l>>>0<3||(0|d)!=(gf[0|(E=z+((P=z>>>0<O>>>0)?cf:B)|0)]|gf[E+1|0]<<8|(gf[E+2|0]<<16|gf[E+3|0]<<24))))){if(H=wf[f+262160>>2],(l=(a=P?V:_)-3|0)>>>0<=(n=C=E+4|0)>>>0)break o;for(;!(b=d^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24)));)if(!((n=n+4|0)>>>0<l>>>0))break o;n=(Rf(b)>>>3|0)+n|0;break k}}b=l-A|0;break b}if(b=d,!(a>>>0<=n>>>0)){for(;;){if(gf[0|n]!=(255&b))break k;if(b=b>>>8|0,(0|a)==(0|(n=n+1|0)))break}n=a}}if(A=H+cf|0,W=4+(n-C|0)|0,O>>>0<=z>>>0)a=h;else{if((0|a)==(E+W|0)){b=Cf(d,W<<3);k:{o:if(!(D>>>0<=(n=h)>>>0)){for(;!(a=(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))^b);)if(!((n=n+4|0)>>>0<D>>>0))break o;n=(Rf(a)>>>3|0)+n|0;break k}if(!(_>>>0<=n>>>0)){for(;;){if(gf[0|n]!=(255&b))break k;if(b=b>>>8|0,(0|_)==(0|(n=n+1|0)))break}n=_}}W=(W-h|0)+n|0}a=A}for(wf[Z>>2]=d,l=a+4|0,b=E;l>>>0<=(n=b)>>>0&&(0|d)==(gf[0|(b=n-4|0)]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24)););k:if(!(n>>>0<=a>>>0)&&(l=kf,(0|tf)==gf[0|(b=n-1|0)]))for(;;){if((n=b)>>>0<=a>>>0){n=a;break k}if(l=l-1|0,gf[0|(b=n-1|0)]!=gf[0|l])break}if(!(O>>>0<=H>>>0|P|(0|h)!=(E-(I=E-n|0)|0))){for(E=Cf(d,0-I<<3),l=A+4|0,a=(wf[Z>>2]=E)>>>24|0,b=V;l>>>0<=(n=b)>>>0&&(0|E)==(gf[0|(b=n-4|0)]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24)););k:if(!(n>>>0<=A>>>0)&&(l=g,(0|a)==gf[0|(b=n-1|0)]))for(;;){if((n=b)>>>0<=A>>>0)break k;if(l=l-1|0,gf[0|(b=n-1|0)]!=gf[0|l])break}I=(I+V|0)-n|0}if(c>>>0<W>>>0|(a=(z-(l=T>>>0<(a=z-I|0)>>>0?a:T)|0)+W|0)>>>0<c>>>0){if(I=2,!((b=O)+(-1^l)>>>0<3)){if((b=a>>>0<c>>>0?a:c)>>>0<=u>>>0)a=R,b=u;else if(65535<(S-(a=l+B|0)|0))break a;if(l>>>0<(n=Sf[131072+(((65535&l)<<1)+f|0)>>1])>>>0){R=a,u=b;break a}R=a,u=b,b=l-n|0}}else I=2,b=O+(-1^(a=(z-c|0)+W|0))>>>0<3?O:a}if(l=b,!s)break a;if(!(T>>>0<=l>>>0))break}a:if(!(!s|65534<K-T>>>0||(A=wf[uf+(Bf(gf[0|S]|gf[S+1|0]<<8|(gf[S+2|0]<<16|gf[S+3|0]<<24),-1640531535)>>>15&131068)>>2],l=wf[uf+262148>>2],65535<K-(h=(T+A|0)-(c=wf[uf+262144>>2]-l|0)|0)>>>0)))for(;;){if(s=s-1|0,(0|d)==(gf[0|(a=l+A|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24))){n=a+4|0,O=h+B|0;b:{if(!((rf=(V=_>>>0<(a=(c-A|0)+S|0)>>>0?_:a)-3|0)>>>0<=(a=b=v)>>>0)){if(a=(gf[0|v]|gf[v+1|0]<<8|(gf[v+2|0]<<16|gf[v+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))){a=Rf(a)>>>3|0;break b}n=n+4|0,a=J}if((b=a)>>>0<rf>>>0)for(;;){if(a=(gf[0|b]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))){a=((Rf(a)>>>3|0)+b|0)-v|0;break b}if(n=n+4|0,!((b=b+4|0)>>>0<rf>>>0))break}(gf[0|n]|gf[n+1|0]<<8)!=(gf[0|b]|gf[b+1|0]<<8)|V-1>>>0<=b>>>0||(n=n+2|0,b=b+2|0),b>>>0<V>>>0&&(b=gf[0|n]==gf[0|b]?b+1|0:b),a=b-v|0}R=(b=(0|u)<(0|(a=a+4|0)))?O:R,u=b?a:u}if(!s)break a;if(A=A-(a=Sf[131072+(uf+((65535&A)<<1)|0)>>1])|0,!(K-(h=h-a|0)>>>0<65536))break}a:if((0|u)<=3)S=S+1|0;else{d=y,of=c=S,O=n=R,v=u;b:{k:{o:{t:for(;;){R=n;n:{if((S=(rf=u)+c|0)>>>0<=Af>>>0){if(a=wf[f+262160>>2],$=(G=(E=S-2|0)-(X=wf[f+262148>>2])|0)>>>0<a+65536>>>0?a:G-65535|0,j=wf[f+262156>>2],V=gf[0|E]|gf[E+1|0]<<8|(gf[E+2|0]<<16|gf[E+3|0]<<24),nf=wf[f+262152>>2],ff=wf[f+262172>>2],(n=wf[f+262164>>2])>>>0<G>>>0)for(;b=(Bf(gf[0|(a=n+X|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24),-1640531535)>>>15&131068)+f|0,a=n-wf[b>>2]|0,vf[131072+(((65535&n)<<1)+f|0)>>1]=a>>>0<65535?a:65535,(n=(wf[b>>2]=n)+1|0)>>>0<G>>>0;);K=E-c|0,wf[f+262164>>2]=G,J=E+8|0,l=E+4|0,bf=c-E|0;s:if((s=wf[(Bf(gf[0|E]|gf[E+1|0]<<8|(gf[E+2|0]<<16|gf[E+3|0]<<24),-1640531535)>>>15&131068)+f>>2])>>>0<$>>>0)B=t,u=rf;else for(uf=(65535&V)==(V>>>16|0)&(0|(cf=V>>>24|0))==(255&V),N=j+nf|0,C=(y=j+X|0)+4|0,P=0-K|0,H=c-1|0,tf=4-l|0,u=rf,B=t,sf=W=0;;){u:{c:{l:{A:{h:{if(j>>>0<=s>>>0){if((0|(b=gf[0|(a=u+H|0)]|gf[a+1|0]<<8))!=(gf[0|(a=(((h=s+X|0)+P|0)+u|0)-1|0)]|gf[a+1|0]<<8)|(0|V)!=(gf[0|h]|gf[h+1|0]<<8|(gf[h+2|0]<<16|gf[h+3|0]<<24)))break u;p:if(K)for(b=(A=(0|(a=y-h|0))<(0|bf)?bf:a)>>31&A,n=0;;){if((0|(a=n))<=(0|A)){a=b;break p}if(gf[E+(n=a-1|0)|0]!=gf[n+h|0])break}else a=0;if(n=h+4|0,(A=D)>>>0<=l>>>0)b=l;else{if(b=(gf[0|l]|gf[l+1|0]<<8|(gf[l+2|0]<<16|gf[l+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24)))break h;n=n+4|0,b=J}if(b>>>0<A>>>0)for(;;){if(A=(gf[0|b]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))){n=((Rf(A)>>>3|0)+b|0)-l|0;break c}if(n=n+4|0,!((b=b+4|0)>>>0<D>>>0))break}(gf[0|n]|gf[n+1|0]<<8)!=(gf[0|b]|gf[b+1|0]<<8)|af>>>0<=b>>>0||(n=n+2|0,b=b+2|0),b>>>0<_>>>0&&(b=gf[0|n]==gf[0|b]?b+1|0:b),n=b-l|0;break c}if((0|V)!=(gf[0|(T=s+nf|0)]|gf[T+1|0]<<8|(gf[T+2|0]<<16|gf[T+3|0]<<24)))break u;if(n=T+4|0,Q=wf[f+262160>>2],!((h=(a=_>>>0<(z=E+(j-s|0)|0)>>>0?_:z)-3|0)>>>0<=(A=b=l)>>>0)){if(b=(gf[0|l]|gf[l+1|0]<<8|(gf[l+2|0]<<16|gf[l+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24)))break A;n=n+4|0,A=J}if((b=A)>>>0<h>>>0)for(;;){if(A=(gf[0|b]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))){b=((Rf(A)>>>3|0)+b|0)-l|0;break l}if(n=n+4|0,!((b=b+4|0)>>>0<h>>>0))break}(gf[0|n]|gf[n+1|0]<<8)!=(gf[0|b]|gf[b+1|0]<<8)|a-1>>>0<=b>>>0||(n=n+2|0,b=b+2|0),b>>>0<a>>>0&&(b=gf[0|n]==gf[0|b]?b+1|0:b),b=b-l|0;break l}n=Rf(b)>>>3|0;break c}b=Rf(b)>>>3|0}if(!((0|a)!=(E+(h=b+4|0)|0)|_>>>0<=z>>>0)){b=y;l:{A:{if((n=a)>>>0<D>>>0){if(b=(gf[0|a]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24))^(gf[0|y]|gf[y+1|0]<<8|(gf[y+2|0]<<16|gf[y+3|0]<<24)))break A;n=a+4|0,b=C}if(n>>>0<D>>>0)for(;;){if(A=(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))^(gf[0|b]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24))){a=((Rf(A)>>>3|0)+n|0)-a|0;break l}if(b=b+4|0,!((n=n+4|0)>>>0<D>>>0))break}(gf[0|b]|gf[b+1|0]<<8)!=(gf[0|n]|gf[n+1|0]<<8)|af>>>0<=n>>>0||(n=n+2|0,b=b+2|0),n>>>0<_>>>0&&(n=gf[0|b]==gf[0|n]?n+1|0:n),a=n-a|0;break l}a=Rf(b)>>>3|0}h=a+h|0}l:if(K)for(a=(A=(0|(a=(Q+nf|0)-T|0))<(0|bf)?bf:a)>>31&A,n=0;;){if((0|(b=n))<=(0|A)){b=a;break l}if(gf[E+(n=b-1|0)|0]!=gf[n+T|0])break}else b=0;if((0|(a=h-b|0))<=(0|u))break u;p=b+E|0,w=(s+X|0)+b|0,u=a;break u}(0|(b=4+(n-a|0)|0))<=(0|u)||(p=a+E|0,w=a+h|0,u=b)}B=B-1|0;u:{c:{l:{A:if(!(!pf|1!=(0|(A=Sf[131072+(((65535&s)<<1)+f|0)>>1])))){if(!W){if(W=1,!uf)break A;h:{p:if(!(D>>>0<=(n=l)>>>0)){for(;!(a=V^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24)));)if(!((n=n+4|0)>>>0<D>>>0))break p;n=(Rf(a)>>>3|0)+n|0;break h}if(b=V,!(_>>>0<=n>>>0)){for(;;){if(gf[0|n]!=(255&b))break h;if(b=b>>>8|0,(0|_)==(0|(n=n+1|0)))break}n=_}}W=2,sf=n+tf|0}if(ef=s-1|0,!(2!=(0|W)|ef>>>0<$>>>0||(W=2,j-s>>>0<3||(0|V)!=(gf[0|(h=ef+((T=ef>>>0<j>>>0)?nf:X)|0)]|gf[h+1|0]<<8|(gf[h+2|0]<<16|gf[h+3|0]<<24))))){if(z=wf[f+262160>>2],(s=(a=T?N:_)-3|0)>>>0<=(n=Q=h+4|0)>>>0)break l;for(;!(b=V^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24)));)if(!((n=n+4|0)>>>0<s>>>0))break l;n=(Rf(b)>>>3|0)+n|0;break c}}s=s-A|0;break u}if(b=V,!(a>>>0<=n>>>0)){for(;;){if(gf[0|n]!=(255&b))break c;if(b=b>>>8|0,(0|a)==(0|(n=n+1|0)))break}n=a}}if(A=z+nf|0,W=4+(n-Q|0)|0,j>>>0<=ef>>>0)a=y;else{if((0|a)==(h+W|0)){b=Cf(V,W<<3);c:{l:if(!(D>>>0<=(n=y)>>>0)){for(;!(a=(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))^b);)if(!((n=n+4|0)>>>0<D>>>0))break l;n=(Rf(a)>>>3|0)+n|0;break c}if(!(_>>>0<=n>>>0)){for(;;){if(gf[0|n]!=(255&b))break c;if(b=b>>>8|0,(0|_)==(0|(n=n+1|0)))break}n=_}}W=(W-y|0)+n|0}a=A}for(wf[Z>>2]=V,s=a+4|0,b=h;s>>>0<=(n=b)>>>0&&(0|V)==(gf[0|(b=n-4|0)]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24)););c:if(!(n>>>0<=a>>>0)&&(s=L,(0|cf)==gf[0|(b=n-1|0)]))for(;;){if((n=b)>>>0<=a>>>0){n=a;break c}if(s=s-1|0,gf[0|(b=n-1|0)]!=gf[0|s])break}if(!(j>>>0<=z>>>0|T|(0|y)!=(h-(a=h-n|0)|0))){for(Q=Cf(V,0-a<<3),s=A+4|0,h=(wf[Z>>2]=Q)>>>24|0,b=N;s>>>0<=(n=b)>>>0&&(0|Q)==(gf[0|(b=n-4|0)]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24)););c:if(!(n>>>0<=A>>>0)&&(s=x,(0|h)==gf[0|(b=n-1|0)]))for(;;){if((n=b)>>>0<=A>>>0)break c;if(s=s-1|0,gf[0|(b=n-1|0)]!=gf[0|s])break}a=(a+N|0)-n|0}if((b=(ef-(h=$>>>0<(a=ef-a|0)>>>0?a:$)|0)+W|0)>>>0<sf>>>0|sf>>>0<W>>>0){if(s=(a=j+(-1^h)>>>0<3)?j:h,W=2,!(a|K)){if((b=b>>>0<sf>>>0?b:sf)>>>0<=u>>>0)a=p,A=w,b=u;else if(65535<((a=E)-(A=h+X|0)|0))break s;if(h>>>0<(n=Sf[131072+(((65535&h)<<1)+f|0)>>1])>>>0){p=a,w=A,u=b;break s}s=h-n|0,p=a,w=A,u=b}}else s=j+(-1^(a=(ef-sf|0)+W|0))>>>0<3?j:a,W=2}if(!B)break s;if(!($>>>0<=s>>>0))break}s:if(!(!B|65534<G-$>>>0||(A=wf[ff+(Bf(gf[0|E]|gf[E+1|0]<<8|(gf[E+2|0]<<16|gf[E+3|0]<<24),-1640531535)>>>15&131068)>>2],Q=wf[ff+262148>>2],65535<G-(C=($+A|0)-(N=wf[ff+262144>>2]-Q|0)|0)>>>0))){if(h=B-1|0,!K)for(;;){if((0|V)==(gf[0|(a=A+Q|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24))){n=a+4|0;u:{if(!((s=(y=_>>>0<(a=E+(N-A|0)|0)>>>0?_:a)-3|0)>>>0<=(a=b=l)>>>0)){if(a=(gf[0|l]|gf[l+1|0]<<8|(gf[l+2|0]<<16|gf[l+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))){a=Rf(a)>>>3|0;break u}n=n+4|0,a=J}if((b=a)>>>0<s>>>0)for(;;){if(a=(gf[0|b]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))){a=((Rf(a)>>>3|0)+b|0)-l|0;break u}if(n=n+4|0,!((b=b+4|0)>>>0<s>>>0))break}(gf[0|n]|gf[n+1|0]<<8)!=(gf[0|b]|gf[b+1|0]<<8)|y-1>>>0<=b>>>0||(n=n+2|0,b=b+2|0),b>>>0<y>>>0&&(b=gf[0|n]==gf[0|b]?b+1|0:b),a=b-l|0}(0|(a=a+4|0))<=(0|u)||(w=C+X|0,p=E,u=a)}if(!h)break s;if(65535<G-(C=C-(a=Sf[131072+(ff+((65535&A)<<1)|0)>>1])|0)>>>0)break s;A=A-a|0,h=h-1|0}for(;;){if((0|V)==(gf[0|(B=A+Q|0)]|gf[B+1|0]<<8|(gf[B+2|0]<<16|gf[B+3|0]<<24))){n=B+4|0;u:{c:{if(!((s=(y=_>>>0<(a=E+(N-A|0)|0)>>>0?_:a)-3|0)>>>0<=(a=b=l)>>>0)){if(a=(gf[0|l]|gf[l+1|0]<<8|(gf[l+2|0]<<16|gf[l+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24)))break c;n=n+4|0,a=J}if((b=a)>>>0<s>>>0)for(;;){if(a=(gf[0|b]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))){b=((Rf(a)>>>3|0)+b|0)-l|0;break u}if(n=n+4|0,!((b=b+4|0)>>>0<s>>>0))break}(gf[0|n]|gf[n+1|0]<<8)!=(gf[0|b]|gf[b+1|0]<<8)|y-1>>>0<=b>>>0||(n=n+2|0,b=b+2|0),b>>>0<y>>>0&&(b=gf[0|n]==gf[0|b]?b+1|0:b),b=b-l|0;break u}b=Rf(a)>>>3|0}for(a=(y=(0|(a=(Q+wf[ff+262156>>2]|0)-B|0))<(0|bf)?bf:a)>>31&y,s=b+4|0,n=0;;){if((0|(b=n))<=(0|y))b=a;else if(gf[E+(n=b-1|0)|0]==gf[n+B|0])continue;break}(0|(a=s-b|0))<=(0|u)||(p=b+E|0,w=(C+X|0)+b|0,u=a)}if(!h)break s;if(65535<G-(C=C-(a=Sf[131072+(ff+((65535&A)<<1)|0)>>1])|0)>>>0)break s;A=A-a|0,h=h-1|0}}if(n=w,(0|u)!=(0|rf))break n}if(o>>>0<9+(((((v=c-m|0)>>>0)/255|0)+d|0)+v|0)>>>0&&k)break o;for(s=d+1|0,15<=v>>>0?(df[0|d]=240,255<=(n=v-15|0)>>>0&&(If(s,255,(b=((a=v-270|0)>>>0)/255|0)+1|0),s=2+(b+d|0)|0,n=a+Bf(b,-255)|0),df[0|s]=n,s=s+1|0):df[0|d]=v<<4,l=s+v|0,n=m,b=s;u=gf[n+4|0]|gf[n+5|0]<<8|(gf[n+6|0]<<16|gf[n+7|0]<<24),a=gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24),df[0|b]=a,df[b+1|0]=a>>>8,df[b+2|0]=a>>>16,df[b+3|0]=a>>>24,df[b+4|0]=u,df[b+5|0]=u>>>8,df[b+6|0]=u>>>16,df[b+7|0]=u>>>24,n=n+8|0,(b=b+8|0)>>>0<l>>>0;);if(a=c-R|0,df[0|l]=a,df[l+1|0]=a>>>8,o>>>0<6+((y=l+2|0)+(((b=rf-4|0)>>>0)/255|0)|0)>>>0&&k)break o;if(a=gf[0|d],15<=b>>>0){df[0|d]=a+15,510<=(n=rf-19|0)>>>0&&(If(y,255,(a=(b=((n=rf-529|0)>>>0)/510|0)<<1)+2|0),y=4+((a+v|0)+s|0)|0,n=n+Bf(b,-510)|0),255<=n>>>0&&(df[0|y]=255,y=y+1|0,n=n-255|0);break b}df[0|d]=a+b,m=S;break a}if(y=(a=p>>>0<c+v>>>0&of>>>0<c>>>0)?of:c,!(((c=p)-y|0)<3)){for(l=a?v:rf,R=a?O:R,J=m;;){cf=(m=l+y|0)+3|0,K=(nf=(0|l)<18?l:18)+y|0;n:{s:{for(;;){17<(0|(a=c-y|0))||(0|(a=(y-c|0)+((u+c|0)-4>>>0<K>>>0?(a+u|0)-4|0:nf)|0))<1?v=u:(v=u-a|0,n=a+n|0,c=a+c|0),w=n;u:{if((S=(p=c)+v|0)>>>0<=Af>>>0){if(a=wf[f+262160>>2],X=(j=(O=S-3|0)-(F=wf[f+262148>>2])|0)>>>0<a+65536>>>0?a:j-65535|0,I=wf[f+262156>>2],E=gf[0|O]|gf[O+1|0]<<8|(gf[O+2|0]<<16|gf[O+3|0]<<24),bf=wf[f+262152>>2],G=wf[f+262172>>2],(n=wf[f+262164>>2])>>>0<j>>>0)for(;b=(Bf(gf[0|(a=n+F|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24),-1640531535)>>>15&131068)+f|0,a=n-wf[b>>2]|0,vf[131072+(((65535&n)<<1)+f|0)>>1]=a>>>0<65535?a:65535,(n=(wf[b>>2]=n)+1|0)>>>0<j>>>0;);ef=O-p|0,wf[f+262164>>2]=j,rf=O+8|0,c=O+4|0,$=p-O|0;c:if((s=wf[(Bf(gf[0|O]|gf[O+1|0]<<8|(gf[O+2|0]<<16|gf[O+3|0]<<24),-1640531535)>>>15&131068)+f>>2])>>>0<X>>>0)B=t,u=v;else for(P=(65535&E)==(E>>>16|0)&(0|(uf=E>>>24|0))==(255&E),of=I+bf|0,C=(V=I+F|0)+4|0,H=0-ef|0,tf=p-1|0,Q=4-c|0,u=v,B=t,sf=W=0;;){l:{A:{h:{p:{m:{if(I>>>0<=s>>>0){if((0|(b=gf[0|(a=u+tf|0)]|gf[a+1|0]<<8))!=(gf[0|(a=(((h=s+F|0)+H|0)+u|0)-1|0)]|gf[a+1|0]<<8)|(0|E)!=(gf[0|h]|gf[h+1|0]<<8|(gf[h+2|0]<<16|gf[h+3|0]<<24)))break l;y:if(ef)for(b=(A=(0|(a=V-h|0))<(0|$)?$:a)>>31&A,n=0;;){if((0|(a=n))<=(0|A)){a=b;break y}if(gf[O+(n=a-1|0)|0]!=gf[n+h|0])break}else a=0;if(n=h+4|0,(A=D)>>>0<=c>>>0)b=c;else{if(b=(gf[0|c]|gf[c+1|0]<<8|(gf[c+2|0]<<16|gf[c+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24)))break m;n=n+4|0,b=rf}if(b>>>0<A>>>0)for(;;){if(A=(gf[0|b]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))){n=((Rf(A)>>>3|0)+b|0)-c|0;break A}if(n=n+4|0,!((b=b+4|0)>>>0<D>>>0))break}(gf[0|n]|gf[n+1|0]<<8)!=(gf[0|b]|gf[b+1|0]<<8)|af>>>0<=b>>>0||(n=n+2|0,b=b+2|0),b>>>0<_>>>0&&(b=gf[0|n]==gf[0|b]?b+1|0:b),n=b-c|0;break A}if((0|E)!=(gf[0|(T=s+bf|0)]|gf[T+1|0]<<8|(gf[T+2|0]<<16|gf[T+3|0]<<24)))break l;if(n=T+4|0,N=wf[f+262160>>2],!((h=(a=_>>>0<(z=O+(I-s|0)|0)>>>0?_:z)-3|0)>>>0<=(A=b=c)>>>0)){if(b=(gf[0|c]|gf[c+1|0]<<8|(gf[c+2|0]<<16|gf[c+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24)))break p;n=n+4|0,A=rf}if((b=A)>>>0<h>>>0)for(;;){if(A=(gf[0|b]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))){b=((Rf(A)>>>3|0)+b|0)-c|0;break h}if(n=n+4|0,!((b=b+4|0)>>>0<h>>>0))break}(gf[0|n]|gf[n+1|0]<<8)!=(gf[0|b]|gf[b+1|0]<<8)|a-1>>>0<=b>>>0||(n=n+2|0,b=b+2|0),b>>>0<a>>>0&&(b=gf[0|n]==gf[0|b]?b+1|0:b),b=b-c|0;break h}n=Rf(b)>>>3|0;break A}b=Rf(b)>>>3|0}if(!((0|a)!=(O+(h=b+4|0)|0)|_>>>0<=z>>>0)){b=V;h:{p:{if((n=a)>>>0<D>>>0){if(b=(gf[0|a]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24))^(gf[0|V]|gf[V+1|0]<<8|(gf[V+2|0]<<16|gf[V+3|0]<<24)))break p;n=a+4|0,b=C}if(n>>>0<D>>>0)for(;;){if(A=(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))^(gf[0|b]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24))){a=((Rf(A)>>>3|0)+n|0)-a|0;break h}if(b=b+4|0,!((n=n+4|0)>>>0<D>>>0))break}(gf[0|b]|gf[b+1|0]<<8)!=(gf[0|n]|gf[n+1|0]<<8)|af>>>0<=n>>>0||(n=n+2|0,b=b+2|0),n>>>0<_>>>0&&(n=gf[0|b]==gf[0|n]?n+1|0:n),a=n-a|0;break h}a=Rf(b)>>>3|0}h=a+h|0}h:if(ef)for(a=(A=(0|(a=(N+bf|0)-T|0))<(0|$)?$:a)>>31&A,n=0;;){if((0|(b=n))<=(0|A)){b=a;break h}if(gf[O+(n=b-1|0)|0]!=gf[n+T|0])break}else b=0;if((0|(a=h-b|0))<=(0|u))break l;M=b+O|0,U=(s+F|0)+b|0,u=a;break l}(0|(b=4+(n-a|0)|0))<=(0|u)||(M=a+O|0,U=a+h|0,u=b)}B=B-1|0;l:{A:{h:{p:if(!(!pf|1!=(0|(A=Sf[131072+(((65535&s)<<1)+f|0)>>1])))){if(!W){if(W=1,!P)break p;m:{y:if(!(D>>>0<=(n=c)>>>0)){for(;!(a=E^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24)));)if(!((n=n+4|0)>>>0<D>>>0))break y;n=(Rf(a)>>>3|0)+n|0;break m}if(b=E,!(_>>>0<=n>>>0)){for(;;){if(gf[0|n]!=(255&b))break m;if(b=b>>>8|0,(0|_)==(0|(n=n+1|0)))break}n=_}}W=2,sf=n+Q|0}if(ff=s-1|0,!(2!=(0|W)|ff>>>0<X>>>0||(W=2,I-s>>>0<3||(0|E)!=(gf[0|(h=ff+((T=ff>>>0<I>>>0)?bf:F)|0)]|gf[h+1|0]<<8|(gf[h+2|0]<<16|gf[h+3|0]<<24))))){if(z=wf[f+262160>>2],(s=(a=T?of:_)-3|0)>>>0<=(n=N=h+4|0)>>>0)break h;for(;!(b=E^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24)));)if(!((n=n+4|0)>>>0<s>>>0))break h;n=(Rf(b)>>>3|0)+n|0;break A}}s=s-A|0;break l}if(b=E,!(a>>>0<=n>>>0)){for(;;){if(gf[0|n]!=(255&b))break A;if(b=b>>>8|0,(0|a)==(0|(n=n+1|0)))break}n=a}}if(A=z+bf|0,W=4+(n-N|0)|0,I>>>0<=ff>>>0)a=V;else{if((0|a)==(h+W|0)){b=Cf(E,W<<3);A:{h:if(!(D>>>0<=(n=V)>>>0)){for(;!(a=(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))^b);)if(!((n=n+4|0)>>>0<D>>>0))break h;n=(Rf(a)>>>3|0)+n|0;break A}if(!(_>>>0<=n>>>0)){for(;;){if(gf[0|n]!=(255&b))break A;if(b=b>>>8|0,(0|_)==(0|(n=n+1|0)))break}n=_}}W=(W-V|0)+n|0}a=A}for(wf[Z>>2]=E,s=a+4|0,b=h;s>>>0<=(n=b)>>>0&&(0|E)==(gf[0|(b=n-4|0)]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24)););A:if(!(n>>>0<=a>>>0)&&(s=Y,(0|uf)==gf[0|(b=n-1|0)]))for(;;){if((n=b)>>>0<=a>>>0){n=a;break A}if(s=s-1|0,gf[0|(b=n-1|0)]!=gf[0|s])break}if(!(I>>>0<=z>>>0|T|(0|V)!=(h-(a=h-n|0)|0))){for(N=Cf(E,0-a<<3),s=A+4|0,h=(wf[Z>>2]=N)>>>24|0,b=of;s>>>0<=(n=b)>>>0&&(0|N)==(gf[0|(b=n-4|0)]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24)););A:if(!(n>>>0<=A>>>0)&&(s=hf,(0|h)==gf[0|(b=n-1|0)]))for(;;){if((n=b)>>>0<=A>>>0)break A;if(s=s-1|0,gf[0|(b=n-1|0)]!=gf[0|s])break}a=(a+of|0)-n|0}if((b=(ff-(h=X>>>0<(a=ff-a|0)>>>0?a:X)|0)+W|0)>>>0<sf>>>0|sf>>>0<W>>>0){if(s=(a=I+(-1^h)>>>0<3)?I:h,W=2,!(a|ef)){if((b=b>>>0<sf>>>0?b:sf)>>>0<=u>>>0)a=M,A=U,b=u;else if(65535<((a=O)-(A=h+F|0)|0))break c;if(h>>>0<(n=Sf[131072+(((65535&h)<<1)+f|0)>>1])>>>0){M=a,U=A,u=b;break c}s=h-n|0,M=a,U=A,u=b}}else s=I+(-1^(a=(ff-sf|0)+W|0))>>>0<3?I:a,W=2}if(!B)break c;if(!(X>>>0<=s>>>0))break}c:if(!(!B|65534<j-X>>>0||(A=wf[G+(Bf(gf[0|O]|gf[O+1|0]<<8|(gf[O+2|0]<<16|gf[O+3|0]<<24),-1640531535)>>>15&131068)>>2],of=wf[G+262148>>2],65535<j-(C=(X+A|0)-(V=wf[G+262144>>2]-of|0)|0)>>>0))){if(h=B-1|0,!ef)for(;;){if((0|E)==(gf[0|(a=A+of|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24))){n=a+4|0;l:{if(!((s=(B=_>>>0<(a=O+(V-A|0)|0)>>>0?_:a)-3|0)>>>0<=(a=b=c)>>>0)){if(a=(gf[0|c]|gf[c+1|0]<<8|(gf[c+2|0]<<16|gf[c+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))){a=Rf(a)>>>3|0;break l}n=n+4|0,a=rf}if((b=a)>>>0<s>>>0)for(;;){if(a=(gf[0|b]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))){a=((Rf(a)>>>3|0)+b|0)-c|0;break l}if(n=n+4|0,!((b=b+4|0)>>>0<s>>>0))break}(gf[0|n]|gf[n+1|0]<<8)!=(gf[0|b]|gf[b+1|0]<<8)|B-1>>>0<=b>>>0||(n=n+2|0,b=b+2|0),b>>>0<B>>>0&&(b=gf[0|n]==gf[0|b]?b+1|0:b),a=b-c|0}(0|(a=a+4|0))<=(0|u)||(U=C+F|0,M=O,u=a)}if(!h)break c;if(65535<j-(C=C-(a=Sf[131072+(G+((65535&A)<<1)|0)>>1])|0)>>>0)break c;A=A-a|0,h=h-1|0}for(;;){if((0|E)==(gf[0|(N=A+of|0)]|gf[N+1|0]<<8|(gf[N+2|0]<<16|gf[N+3|0]<<24))){n=N+4|0;l:{A:{if(!((s=(B=_>>>0<(a=O+(V-A|0)|0)>>>0?_:a)-3|0)>>>0<=(a=b=c)>>>0)){if(a=(gf[0|c]|gf[c+1|0]<<8|(gf[c+2|0]<<16|gf[c+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24)))break A;n=n+4|0,a=rf}if((b=a)>>>0<s>>>0)for(;;){if(a=(gf[0|b]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))){b=((Rf(a)>>>3|0)+b|0)-c|0;break l}if(n=n+4|0,!((b=b+4|0)>>>0<s>>>0))break}(gf[0|n]|gf[n+1|0]<<8)!=(gf[0|b]|gf[b+1|0]<<8)|B-1>>>0<=b>>>0||(n=n+2|0,b=b+2|0),b>>>0<B>>>0&&(b=gf[0|n]==gf[0|b]?b+1|0:b),b=b-c|0;break l}b=Rf(a)>>>3|0}for(a=(B=(0|(a=(of+wf[G+262156>>2]|0)-N|0))<(0|$)?$:a)>>31&B,s=b+4|0,n=0;;){if((0|(b=n))<=(0|B))b=a;else if(gf[O+(n=b-1|0)|0]==gf[n+N|0])continue;break}(0|(a=s-b|0))<=(0|u)||(M=b+O|0,U=(C+F|0)+b|0,u=a)}if(!h)break c;if(65535<j-(C=C-(a=Sf[131072+(G+((65535&A)<<1)|0)>>1])|0)>>>0)break c;A=A-a|0,h=h-1|0}}if(c=M,n=U,(0|u)!=(0|v))break u}if(o>>>0<9+(((((A=y-J|0)>>>0)/255|0)+d|0)+A|0)>>>0&&k)break n;for(n=p>>>0<m>>>0,b=p-y|0,u=d+1|0,15<=A>>>0?(df[0|d]=240,255<=(s=A-15|0)>>>0&&(If(s=u,255,(u=((a=A-270|0)>>>0)/255|0)+1|0),s=a+Bf(u,-255)|0,u=2+(u+d|0)|0),df[0|u]=s,u=u+1|0):df[0|d]=A<<4,m=n?b:l,c=u+A|0,n=J,b=u;s=gf[n+4|0]|gf[n+5|0]<<8|(gf[n+6|0]<<16|gf[n+7|0]<<24),a=gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24),df[0|b]=a,df[b+1|0]=a>>>8,df[b+2|0]=a>>>16,df[b+3|0]=a>>>24,df[b+4|0]=s,df[b+5|0]=s>>>8,df[b+6|0]=s>>>16,df[b+7|0]=s>>>24,n=n+8|0,(b=b+8|0)>>>0<c>>>0;);if(a=y-R|0,df[0|c]=a,df[c+1|0]=a>>>8,o>>>0<6+((l=c+2|0)+(((b=m-4|0)>>>0)/255|0)|0)>>>0&&k)break n;if(a=gf[0|d],15<=b>>>0?(df[0|d]=a+15,510<=(n=m-19|0)>>>0&&(If(l,255,(a=(b=((n=m-529|0)>>>0)/510|0)<<1)+2|0),l=4+((a+A|0)+u|0)|0,n=n+Bf(b,-510)|0),255<=n>>>0&&(df[0|l]=255,l=l+1|0,n=n-255|0),df[0|l]=n,l=l+1|0):df[0|d]=a+b,!(!k|9+(((((n=p-(m=m+y|0)|0)>>>0)/255|0)+l|0)+n|0)>>>0<=o>>>0))break k;for(s=l+1|0,15<=n>>>0?(df[0|l]=240,255<=(b=n-15|0)>>>0&&(If(s,255,(b=((a=n-270|0)>>>0)/255|0)+1|0),s=2+(b+l|0)|0,b=a+Bf(b,-255)|0),df[0|s]=b,s=s+1|0):df[0|l]=n<<4,c=n+s|0,n=m,b=s;u=gf[n+4|0]|gf[n+5|0]<<8|(gf[n+6|0]<<16|gf[n+7|0]<<24),a=gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24),df[0|b]=a,df[b+1|0]=a>>>8,df[b+2|0]=a>>>16,df[b+3|0]=a>>>24,df[b+4|0]=u,df[b+5|0]=u>>>8,df[b+6|0]=u>>>16,df[b+7|0]=u>>>24,n=n+8|0,(b=b+8|0)>>>0<c>>>0;);if(a=p-w|0,df[0|c]=a,df[c+1|0]=a>>>8,!(!k|6+((y=c+2|0)+(((b=v-4|0)>>>0)/255|0)|0)>>>0<=o>>>0))break k;if(a=gf[0|l],15<=b>>>0){df[0|l]=a+15,510<=(n=v-19|0)>>>0&&(If(y,255,(a=(b=((n=v-529|0)>>>0)/510|0)<<1)+2|0),y=4+(((a+p|0)-m|0)+s|0)|0,n=n+Bf(b,-510)|0),255<=n>>>0&&(df[0|y]=255,y=y+1|0,n=n-255|0);break b}df[0|l]=a+b,m=S;break a}if(cf>>>0<=c>>>0)break s;if(!(c>>>0<m>>>0))break}if(m>>>0<=p>>>0||(3<(0|(v=v-(a=m-p|0)|0))?(w=a+w|0,p=m):(p=c,w=n,v=u)),o>>>0<9+(((((A=y-J|0)>>>0)/255|0)+d|0)+A|0)>>>0&&k)break n;for(a=d+1|0,15<=A>>>0?(df[0|d]=240,255<=(b=A-15|0)>>>0&&(If(b=a,255,(s=((a=A-270|0)>>>0)/255|0)+1|0),b=a+Bf(s,-255)|0,a=2+(s+d|0)|0),df[0|a]=b,a=a+1|0):df[0|d]=A<<4,S=a+A|0,b=J,s=a;U=gf[b+4|0]|gf[b+5|0]<<8|(gf[b+6|0]<<16|gf[b+7|0]<<24),M=gf[0|b]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24),df[0|s]=M,df[s+1|0]=M>>>8,df[s+2|0]=M>>>16,df[s+3|0]=M>>>24,df[s+4|0]=U,df[s+5|0]=U>>>8,df[s+6|0]=U>>>16,df[s+7|0]=U>>>24,b=b+8|0,(s=s+8|0)>>>0<S>>>0;);if(b=y-R|0,df[0|S]=b,df[S+1|0]=b>>>8,o>>>0<6+((b=S+2|0)+(((M=l-4|0)>>>0)/255|0)|0)>>>0&&k)break n;s=gf[0|d],d=15<=M>>>0?(df[0|d]=s+15,510<=(s=l-19|0)>>>0&&(If(R=b,255,(b=(s=((M=l-529|0)>>>0)/510|0)<<1)+2|0),s=M+Bf(s,-510)|0,b=4+((b+A|0)+a|0)|0),255<=s>>>0&&(s=s-(df[0|b]=255)|0,b=b+1|0),df[0|b]=s,b+1|0):(df[0|d]=s+M,b),M=c,U=n,of=p,O=w;continue t}if(m>>>0<=p>>>0?(A=l,l=v):(l=v,17<(0|(A=p-y|0))||(0|(a=(A=((l=v)+p|0)-4>>>0<K>>>0?(l+A|0)-4|0:nf)+(y-p|0)|0))<1||(w=a+w|0,p=a+p|0,l=v-a|0)),!(o>>>0<9+(((((S=y-J|0)>>>0)/255|0)+d|0)+S|0)>>>0&&k)){for(a=d+1|0,15<=S>>>0?(df[0|d]=240,255<=(b=S-15|0)>>>0&&(If(b=a,255,(s=((a=S-270|0)>>>0)/255|0)+1|0),b=a+Bf(s,-255)|0,a=2+(s+d|0)|0),df[0|a]=b,a=a+1|0):df[0|d]=S<<4,U=a+S|0,b=J,s=a;M=gf[b+4|0]|gf[b+5|0]<<8|(gf[b+6|0]<<16|gf[b+7|0]<<24),m=gf[0|b]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24),df[0|s]=m,df[s+1|0]=m>>>8,df[s+2|0]=m>>>16,df[s+3|0]=m>>>24,df[s+4|0]=M,df[s+5|0]=M>>>8,df[s+6|0]=M>>>16,df[s+7|0]=M>>>24,b=b+8|0,(s=s+8|0)>>>0<U>>>0;);if(b=y-R|0,df[0|U]=b,df[U+1|0]=b>>>8,!(o>>>0<6+((b=U+2|0)+(((m=A-4|0)>>>0)/255|0)|0)>>>0&&k)){s=gf[0|d],J=(d=15<=m>>>0?(df[0|d]=s+15,510<=(s=A-19|0)>>>0&&(If(v=b,255,(b=(s=((m=A-529|0)>>>0)/510|0)<<1)+2|0),s=m+Bf(s,-510)|0,b=4+((b+S|0)+a|0)|0),255<=s>>>0&&(s=s-(df[0|b]=255)|0,b=b+1|0),df[0|b]=s,b+1|0):(df[0|d]=s+m,b),A+y|0),y=p,R=w,M=c,U=n;continue}}}break}break}}m=J}l=d}if(y=l,2!=((n=0)|k))break r;break i}df[0|y]=n,y=y+1|0,m=S}if(Af>>>0<S>>>0)break i;B=wf[f+262148>>2]}if(n=lf-m|0,a=(n+240>>>0)/255|0,k&&!((b=1+((a+n|0)+y|0)|0)>>>0<=(a=yf?o+5|0:mf)>>>0)){if(1==((n=0)|k))break r;n=(a=a+(-1^y)|0)-((a+240>>>0)/255|0)|0}k=n+m|0,15<=n>>>0?(df[0|y]=240,a=y+1|0,(b=n-15|0)>>>0<255?df[0|(y=a)]=b:(If(u=a,255,(b=((a=n-270|0)>>>0)/255|0)+1|0),df[0|(y=2+(b+y|0)|0)]=a+Bf(b,-255))):df[0|y]=n<<4,a=Ef(y+1|0,m,n),wf[i>>2]=k-r,n=(a+n|0)-e|0}else{G=gf[f+262170|0],nf=e+a|(wf[i>>2]=0),lf=(yf=2==(0|k))?nf-5|0:nf,S=e;i:if(!((Af=(K=s+(M=r)|0)-12|0)>>>0<M>>>0)){for(mf=G?-1:0,T=(a=wf[b+1768>>2])>>>0<4095?a:4095,$=(v=K-5|0)-1|0,_=v-3|0,rf=V=D=E=O=of=65596+Z|3,ff=f+131072|0,cf=(0|u)<12,x=r;;){if(c=x-B|0,m=wf[f+262160>>2],w=c>>>0<m+65536>>>0,s=c-65535|0,kf=wf[f+262156>>2],R=kf+B|0,J=gf[0|x]|gf[x+1|0]<<8|(gf[x+2|0]<<16|gf[x+3|0]<<24),X=wf[f+262152>>2],af=wf[f+262172>>2],n=wf[f+262164>>2],!(c>>>0<=n>>>0)&&(u=(-1^n)+x|0,(x-n|0)-B&1&&(b=(Bf(gf[0|(a=n+B|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24),-1640531535)>>>15&131068)+f|0,a=n-wf[b>>2]|0,vf[131072+(((65535&n)<<1)+f|0)>>1]=a>>>0<65535?a:65535,n=(wf[b>>2]=n)+1|0),(0|u)!=(0|B)))for(;b=(Bf(gf[0|(a=n+B|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24),-1640531535)>>>15&131068)+f|0,a=n-wf[b>>2]|0,vf[ff+((65535&n)<<1)>>1]=a>>>0<65535?a:65535,wf[b>>2]=n,b=(Bf(gf[0|(a=(u=n+1|0)+B|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24),-1640531535)>>>15&131068)+f|0,a=u-wf[b>>2]|0,vf[ff+((65535&u)<<1)>>1]=a>>>0<65535?a:65535,wf[b>>2]=u,(n=n+2|0)>>>0<c>>>0;);for(P=w?m:s,g=x-M|0,wf[f+262164>>2]=c,Q=(65535&J)==(J>>>16|0)&(0|(tf=J>>>24|0))==(255&J),N=(p=0)-x|0,h=X+kf|0,y=R+4|0,A=x+8|0,w=x+4|0,hf=x-1|0,Y=Bf(gf[0|x]|gf[x+1|0]<<8|(gf[x+2|0]<<16|gf[x+3|0]<<24),-1640531535)>>>17<<2,u=wf[Y+f>>2],m=3,l=s=I=0,U=t;;){a:if(!(!U|u>>>0<P>>>0)){b:if(!(c-u>>>(C=0)<8&&G)){k:{o:{t:{n:{if(kf>>>0<=u>>>0){if((0|(b=gf[0|(a=m+hf|0)]|gf[a+1|0]<<8))!=(gf[0|(a=((d=u+B|0)+m|0)-1|0)]|gf[a+1|0]<<8)|(0|J)!=(gf[0|d]|gf[d+1|0]<<8|(gf[d+2|0]<<16|gf[d+3|0]<<24)))break b;if(n=d+4|0,(b=_)>>>0<=w>>>0)a=w;else{if(a=(gf[0|w]|gf[w+1|0]<<8|(gf[w+2|0]<<16|gf[w+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24)))break n;n=n+4|0,a=A}if(a>>>0<b>>>0)for(;;){if(b=(gf[0|a]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))){n=((Rf(b)>>>3|0)+a|0)-w|0;break k}if(n=n+4|0,!((a=a+4|0)>>>0<_>>>0))break}(gf[0|n]|gf[n+1|0]<<8)!=(gf[0|a]|gf[a+1|0]<<8)|$>>>0<=a>>>0||(n=n+2|0,a=a+2|0),a>>>0<v>>>0&&(a=gf[0|n]==gf[0|a]?a+1|0:a),n=a-w|0;break k}if((0|J)!=(gf[0|(a=u+X|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24)))break b;if(n=a+4|0,!((C=(b=v>>>0<(L=(kf-u|0)+x|0)>>>0?v:L)-3|0)>>>0<=(d=a=w)>>>0)){if(a=(gf[0|w]|gf[w+1|0]<<8|(gf[w+2|0]<<16|gf[w+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24)))break t;n=n+4|0,d=A}if((a=d)>>>0<C>>>0)for(;;){if(d=(gf[0|a]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))){a=((Rf(d)>>>3|0)+a|0)-w|0;break o}if(n=n+4|0,!((a=a+4|0)>>>0<C>>>0))break}(gf[0|n]|gf[n+1|0]<<8)!=(gf[0|a]|gf[a+1|0]<<8)|b-1>>>0<=a>>>0||(n=n+2|0,a=a+2|0),a>>>0<b>>>0&&(a=gf[0|n]==gf[0|a]?a+1|0:a),a=a-w|0;break o}n=Rf(a)>>>3|0;break k}a=Rf(a)>>>3|0}if(F=u+B|0,!((0|b)!=((C=a+4|0)+x|0)|v>>>0<=L>>>0)){a=R;o:{t:{if((n=b)>>>0<_>>>0){if(a=(gf[0|b]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24))^(gf[0|R]|gf[R+1|0]<<8|(gf[R+2|0]<<16|gf[R+3|0]<<24)))break t;n=b+4|0,a=y}if(n>>>0<_>>>0)for(;;){if(d=(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))^(gf[0|a]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24))){a=((Rf(d)>>>3|0)+n|0)-b|0;break o}if(a=a+4|0,!((n=n+4|0)>>>0<_>>>0))break}(gf[0|a]|gf[a+1|0]<<8)!=(gf[0|n]|gf[n+1|0]<<8)|$>>>0<=n>>>0||(n=n+2|0,a=a+2|0),n>>>0<v>>>0&&(n=gf[0|a]==gf[0|n]?n+1|0:n),a=n-b|0;break o}a=Rf(a)>>>3|0}C=a+C|0}p=(a=(0|m)<(0|C))?F:p,m=a?C:m;break b}m=(a=(0|m)<(0|(C=n+4|0)))?C:m,p=a?d:p}U=U-1|0;b:{if(!((0|m)!=(0|C)|(0|C)<4|c>>>0<u+m>>>0)){for(L=C-3|0,n=0,b=16,a=1;a=(H=a>>>0<(d=Sf[131072+(((n+u&65535)<<1)+f|0)>>1])>>>0)?d:a,l=H?n:l,d=b>>4,b=H?16:b+1|0,(0|(n=n+d|0))<(0|L););if(n=(b=u>>>0<a>>>0)?0:a,u=u-((a=1<a>>>0)?n:0)|0,a){n=b?3:2,m=C;break b}}k:{o:{t:if(!(1!=Sf[131072+(((65535&u)<<1)+f|0)>>1]|l)){if(!s){if(s=1,!Q)break t;n:{s:if(!(_>>>0<=(n=w)>>>0)){for(;!(a=J^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24)));)if(!((n=n+4|0)>>>0<_>>>0))break s;n=(Rf(a)>>>3|0)+n|0;break n}if(a=J,!(v>>>0<=n>>>0)){for(;;){if(gf[0|n]!=(255&a))break n;if(a=a>>>8|0,(0|v)==(0|(n=n+1|0)))break}n=v}}I=n+N|0,s=2}if(H=u-1|0,!(2!=(0|s)|H>>>0<P>>>0||(s=2,kf-u>>>0<3||(0|J)!=(gf[0|(d=H+((L=H>>>0<kf>>>0)?X:B)|0)]|gf[d+1|0]<<8|(gf[d+2|0]<<16|gf[d+3|0]<<24))))){if(C=wf[f+262160>>2],(u=(b=L?h:v)-3|0)>>>0<=(n=s=d+4|0)>>>0)break o;for(;!(a=J^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24)));)if(!((n=n+4|0)>>>0<u>>>0))break o;n=(Rf(a)>>>3|0)+n|0;break k}}u=u-Sf[131072+(((u+l&65535)<<1)+f|0)>>1]|0,n=0;break b}if(a=J,!(b>>>0<=n>>>0)){for(;;){if(gf[0|n]!=(255&a))break k;if(a=a>>>8|0,(0|b)==(0|(n=n+1|0)))break}n=b}}if(l=C+X|0,u=4+(n-s|0)|0,kf>>>0<=H>>>0)a=R;else{if((0|b)==(u+d|0)){a=Cf(J,u<<3);k:{o:if(!(_>>>0<=(n=R)>>>0)){for(;!(b=(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))^a);)if(!((n=n+4|0)>>>0<_>>>0))break o;n=(Rf(b)>>>3|0)+n|0;break k}if(!(v>>>0<=n>>>0)){for(;;){if(gf[0|n]!=(255&a))break k;if(a=a>>>8|0,(0|v)==(0|(n=n+1|0)))break}n=v}}u=(u-R|0)+n|0}a=l}for(wf[65596+Z>>2]=J,b=(s=a)+4|0,a=d;b>>>0<=(n=a)>>>0&&(0|J)==(gf[0|(a=n-4|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24)););k:if(!(n>>>0<=s>>>0)&&(b=rf,(0|tf)==gf[0|(a=n-1|0)]))for(;;){if((n=a)>>>0<=s>>>0){n=s;break k}if(b=b-1|0,gf[0|(a=n-1|0)]!=gf[0|b])break}if(!(kf>>>0<=C>>>0|L|(0|R)!=(d-(s=d-n|0)|0))){for(C=Cf(J,0-s<<3),b=l+4|0,d=(wf[65596+Z>>2]=C)>>>24|0,a=h;b>>>0<=(n=a)>>>0&&(0|C)==(gf[0|(a=n-4|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24)););k:if(!(n>>>0<=l>>>0)&&(b=V,(0|d)==gf[0|(a=n-1|0)]))for(;;){if((n=a)>>>0<=l>>>0)break k;if(b=b-1|0,gf[0|(a=n-1|0)]!=gf[0|b])break}s=(s+h|0)-n|0}if((a=(H-(d=P>>>0<(a=H-s|0)>>>0?a:P)|0)+u|0)>>>0<I>>>0|I>>>0<u>>>0)if(n=2,kf+(-1^d)>>>(l=0)<3)s=2,u=kf;else{if((b=a>>>0<I>>>0?a:I)>>>0<=m>>>0)s=p,b=m;else if(65535<(x-(s=d+B|0)|0))break a;if(d>>>0<(a=Sf[131072+(((65535&d)<<1)+f|0)>>1])>>>0){p=s,m=b;break a}u=d-a|0,p=s,s=2,m=b}else u=kf+(-1^(a=(H-I|0)+u|0))>>>0<3?kf:a,l=0,s=n=2}if(3!=(0|n))continue}break}a:if(!(!U|65534<c-P>>>0||(l=wf[af+Y>>2],R=wf[af+262148>>2],65535<c-(s=(P+l|0)-(u=wf[af+262144>>2]-R|0)|0)>>>0)))for(;;){if(U=U-1|0,(0|J)==(gf[0|(a=l+R|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24))){n=a+4|0,d=s+B|0;b:{if(!((y=(h=v>>>0<(b=(u-l|0)+x|0)>>>0?v:b)-3|0)>>>0<=(b=a=w)>>>0)){if(a=(gf[0|w]|gf[w+1|0]<<8|(gf[w+2|0]<<16|gf[w+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))){a=Rf(a)>>>3|0;break b}n=n+4|0,b=A}if((a=b)>>>0<y>>>0)for(;;){if(b=(gf[0|a]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))){a=((Rf(b)>>>3|0)+a|0)-w|0;break b}if(n=n+4|0,!((a=a+4|0)>>>0<y>>>0))break}(gf[0|n]|gf[n+1|0]<<8)!=(gf[0|a]|gf[a+1|0]<<8)|h-1>>>0<=a>>>0||(n=n+2|0,a=a+2|0),a>>>0<h>>>0&&(a=gf[0|n]==gf[0|a]?a+1|0:a),a=a-w|0}p=(b=(0|m)<(0|(a=a+4|0)))?d:p,m=b?a:m}if(!U)break a;if(l=l-(a=Sf[131072+(af+((65535&l)<<1)|0)>>1])|0,!(c-(s=s-a|0)>>>0<65536))break}a:{b:{k:{o:{t:{if(4<=(0|m)){if(w=x-p|0,T>>>0<(I=G&&m-19>>>0<18?18:m)>>>0)break t;if(u=14<(0|g))break o;a=g+1|0,b=g;break k}x=x+1|0;break b}if(lf>>>0<9+(g+(((g>>>0)/255|0)+S|0)|0)>>>0&&k)break a;for(l=S+1|0,15<=g>>>0?(df[0|S]=240,255<=(n=g-15|0)>>>0&&(If(l,255,(b=((a=(x-M|0)-270|0)>>>0)/255|0)+1|0),l=2+(b+S|0)|0,n=a+Bf(b,-255)|0),df[0|l]=n,l=l+1|0):df[0|S]=g<<4,s=l+g|0,n=M,a=l;u=gf[n+4|0]|gf[n+5|0]<<8|(gf[n+6|0]<<16|gf[n+7|0]<<24),b=gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24),df[0|a]=b,df[a+1|0]=b>>>8,df[a+2|0]=b>>>16,df[a+3|0]=b>>>24,df[a+4|0]=u,df[a+5|0]=u>>>8,df[a+6|0]=u>>>16,df[a+7|0]=u>>>24,n=n+8|0,(a=a+8|0)>>>0<s>>>0;);if(df[0|s]=w,df[s+1|0]=w>>>8,lf>>>0<6+((n=s+2|0)+(((b=I-4|0)>>>0)/255|0)|0)>>>0&&k)break a;if(a=gf[0|S],15<=b>>>0){df[0|S]=a+15,510<=(a=I-19|0)>>>0&&(If(n,255,(a=(b=((u=I-529|0)>>>0)/510|0)<<1)+2|0),n=4+((a+g|0)+l|0)|0,a=u+Bf(b,-510)|0),255<=a>>>0&&(df[0|n]=255,n=n+1|0,a=a-255|0),df[0|n]=a,S=n+1|0,M=x=I+x|0;break b}df[0|S]=a+b,M=x=I+x|0,S=n;break b}b=(a=g+1|0)+((g-15|0)/255|0)|0}wf[12+Z>>2]=g,wf[4+Z>>2]=0,wf[8+Z>>2]=1,wf[Z>>2]=b,wf[28+Z>>2]=a,wf[20+Z>>2]=0,wf[24+Z>>2]=1,14<(0|(b=a))&&(b=1+(((a-15|0)/255|0)+a|0)|0),wf[(n=Z)+16>>2]=b,b=g+2|0;k:{if(13<=(0|g))wf[44+Z>>2]=b,wf[36+Z>>2]=0,wf[40+Z>>2]=1,l=g+3|0,wf[32+Z>>2]=l+((g-13|0)/255|0);else if(wf[44+Z>>2]=b,wf[36+Z>>2]=0,wf[40+Z>>2]=1,wf[32+Z>>2]=b,l=15,12!=(0|g)){b=l=g+3|0;break k}b=4+(g+((g-12|0)/255|0)|0)|0}if(wf[60+Z>>2]=l,wf[52+Z>>2]=0,wf[56+Z>>2]=1,wf[48+Z>>2]=b,u)for(s=(a=((g-15|0)/255|0)+a|0)+(n=4)|0,u=a+3|0;a=u,wf[(b=(n<<4)+Z|0)+12>>2]=g,wf[b+4>>2]=w,19<=(wf[b+8>>2]=n)>>>0&&(a=s+((n-19|0)/255|0)|0),wf[b>>2]=a,a=(0|n)==(0|I),n=n+1|0,!a;);else for(u=l+1|0,n=4;a=l,wf[(b=(n<<4)+Z|0)+12>>2]=g,wf[b+4>>2]=w,19<=(wf[b+8>>2]=n)>>>0&&(a=u+((n-19|0)/255|0)|0),wf[b>>2]=a,a=(0|n)==(0|I),n=n+1|0,!a;);wf[(b=(I<<4)+Z|0)+28>>2]=1,wf[b+20>>2]=0,wf[b+24>>2]=1,wf[b+36>>2]=0,wf[b+40>>2]=1,wf[b+44>>2]=2,wf[b+60>>2]=3,wf[b+52>>2]=0,wf[b+56>>2]=1,a=wf[b>>2],wf[b+16>>2]=a+1,wf[b+32>>2]=a+2,wf[b+48>>2]=a+3;k:{o:if(!((0|I)<2|Af>>>0<(F=x+1|0)>>>0))for(z=-1^B,J=1;;){bf=wf[(uf=(a=J<<4)+Z|0)>>2],pf=wf[((kf=J+1|0)<<4)+Z>>2];t:{n:{s:{u:{if(!cf){if((0|bf)<(0|pf)|wf[(a+Z|0)+64>>2]>=(bf+3|0))break u;U=c;break t}if((0|bf)<(0|pf))break s;U=c;break t}if(U=F-B|0,w=wf[f+262160>>2],s=U>>>0<w+65536>>>0,u=U-65535|0,L=wf[f+262156>>2],R=L+B|0,y=gf[0|F]|gf[F+1|0]<<8|(gf[F+2|0]<<16|gf[F+3|0]<<24),!(U>>>0<=c>>>0)&&((F-B|0)-(n=c)&1&&(b=(Bf(gf[0|(a=c+B|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24),-1640531535)>>>15&131068)+f|0,a=c-wf[b>>2]|0,vf[131072+(((65535&c)<<1)+f|0)>>1]=a>>>0<65535?a:65535,n=(wf[b>>2]=c)+1|0),(F+z|0)!=(0|c)))for(;b=(Bf(gf[0|(a=n+B|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24),-1640531535)>>>15&131068)+f|0,a=n-wf[b>>2]|0,vf[ff+((65535&n)<<1)>>1]=a>>>0<65535?a:65535,wf[b>>2]=n,b=(Bf(gf[0|(a=(c=n+1|0)+B|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24),-1640531535)>>>15&131068)+f|0,a=c-wf[b>>2]|0,vf[ff+((65535&c)<<1)>>1]=a>>>0<65535?a:65535,wf[b>>2]=c,(n=n+2|0)>>>0<U>>>0;);for(q=s?w:u,wf[f+262164>>2]=U,tf=(65535&y)==(y>>>16|0)&(0|(H=y>>>24|0))==(255&y),Q=(w=0)-F|0,d=L+X|0,h=R+4|0,A=F+8|0,p=F+4|0,N=F-1|0,hf=Bf(gf[0|F]|gf[F+1|0]<<8|(gf[F+2|0]<<16|gf[F+3|0]<<24),-1640531535)>>>17<<2,u=wf[hf+f>>2],c=3,l=s=j=0,C=t;;){u:if(!(!C|u>>>0<q>>>0)){c:if(!(U-u>>>(m=0)<8&&G)){l:{A:{h:{p:{if(L>>>0<=u>>>0){if((0|(b=gf[0|(a=c+N|0)]|gf[a+1|0]<<8))!=(gf[0|(a=((g=u+B|0)+c|0)-1|0)]|gf[a+1|0]<<8)|(0|y)!=(gf[0|g]|gf[g+1|0]<<8|(gf[g+2|0]<<16|gf[g+3|0]<<24)))break c;if(n=g+4|0,(b=_)>>>0<=p>>>0)a=p;else{if(a=(gf[0|p]|gf[p+1|0]<<8|(gf[p+2|0]<<16|gf[p+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24)))break p;n=n+4|0,a=A}if(a>>>0<b>>>0)for(;;){if(b=(gf[0|a]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))){n=((Rf(b)>>>3|0)+a|0)-p|0;break l}if(n=n+4|0,!((a=a+4|0)>>>0<_>>>0))break}(gf[0|n]|gf[n+1|0]<<8)!=(gf[0|a]|gf[a+1|0]<<8)|$>>>0<=a>>>0||(n=n+2|0,a=a+2|0),a>>>0<v>>>0&&(a=gf[0|n]==gf[0|a]?a+1|0:a),n=a-p|0;break l}if((0|y)!=(gf[0|(a=u+X|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24)))break c;if(n=a+4|0,!((g=(b=v>>>0<(Y=(L-u|0)+F|0)>>>0?v:Y)-3|0)>>>0<=(m=a=p)>>>0)){if(a=(gf[0|p]|gf[p+1|0]<<8|(gf[p+2|0]<<16|gf[p+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24)))break h;n=n+4|0,m=A}if((a=m)>>>0<g>>>0)for(;;){if(m=(gf[0|a]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))){a=((Rf(m)>>>3|0)+a|0)-p|0;break A}if(n=n+4|0,!((a=a+4|0)>>>0<g>>>0))break}(gf[0|n]|gf[n+1|0]<<8)!=(gf[0|a]|gf[a+1|0]<<8)|b-1>>>0<=a>>>0||(n=n+2|0,a=a+2|0),a>>>0<b>>>0&&(a=gf[0|n]==gf[0|a]?a+1|0:a),a=a-p|0;break A}n=Rf(a)>>>3|0;break l}a=Rf(a)>>>3|0}if(W=u+B|0,!((0|b)!=(F+(m=a+4|0)|0)|v>>>0<=Y>>>0)){a=R;A:{h:{if((n=b)>>>0<_>>>0){if(a=(gf[0|b]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24))^(gf[0|R]|gf[R+1|0]<<8|(gf[R+2|0]<<16|gf[R+3|0]<<24)))break h;n=b+4|0,a=h}if(n>>>0<_>>>0)for(;;){if(g=(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))^(gf[0|a]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24))){a=((Rf(g)>>>3|0)+n|0)-b|0;break A}if(a=a+4|0,!((n=n+4|0)>>>0<_>>>0))break}(gf[0|a]|gf[a+1|0]<<8)!=(gf[0|n]|gf[n+1|0]<<8)|$>>>0<=n>>>0||(n=n+2|0,a=a+2|0),n>>>0<v>>>0&&(n=gf[0|a]==gf[0|n]?n+1|0:n),a=n-b|0;break A}a=Rf(a)>>>3|0}m=a+m|0}w=(a=(0|c)<(0|m))?W:w,c=a?m:c;break c}c=(a=(0|c)<(0|(m=n+4|0)))?m:c,w=a?g:w}C=C-1|0;c:{if(!((0|c)!=(0|m)|(0|m)<4|U>>>0<u+c>>>0)){for(Y=m-3|0,n=0,b=16,a=1;a=(P=a>>>0<(g=Sf[131072+(((n+u&65535)<<1)+f|0)>>1])>>>0)?g:a,l=P?n:l,g=b>>4,b=P?16:b+1|0,(0|(n=n+g|0))<(0|Y););if(n=(b=u>>>0<a>>>0)?0:a,u=u-((a=1<a>>>0)?n:0)|0,a){n=b?3:2,c=m;break c}}l:{A:{h:if(!(1!=Sf[131072+(((65535&u)<<1)+f|0)>>1]|l)){if(!s){if(s=1,!tf)break h;p:{m:if(!(_>>>0<=(n=p)>>>0)){for(;!(a=y^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24)));)if(!((n=n+4|0)>>>0<_>>>0))break m;n=(Rf(a)>>>3|0)+n|0;break p}if(a=y,!(v>>>0<=n>>>0)){for(;;){if(gf[0|n]!=(255&a))break p;if(a=a>>>8|0,(0|v)==(0|(n=n+1|0)))break}n=v}}j=n+Q|0,s=2}if(P=u-1|0,!(2!=(0|s)|P>>>0<q>>>0||(s=2,L-u>>>0<3||(0|y)!=(gf[0|(g=P+((Y=P>>>0<L>>>0)?X:B)|0)]|gf[g+1|0]<<8|(gf[g+2|0]<<16|gf[g+3|0]<<24))))){if(l=wf[f+262160>>2],(u=(b=Y?d:v)-3|0)>>>0<=(n=s=g+4|0)>>>0)break A;for(;!(a=y^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24)));)if(!((n=n+4|0)>>>0<u>>>0))break A;n=(Rf(a)>>>3|0)+n|0;break l}}u=u-Sf[131072+(((u+l&65535)<<1)+f|0)>>1]|0,n=0;break c}if(a=y,!(b>>>0<=n>>>0)){for(;;){if(gf[0|n]!=(255&a))break l;if(a=a>>>8|0,(0|b)==(0|(n=n+1|0)))break}n=b}}if(m=l+X|0,u=4+(n-s|0)|0,L>>>0<=P>>>0)a=R;else{if((0|b)==(u+g|0)){a=Cf(y,u<<3);l:{A:if(!(_>>>0<=(n=R)>>>0)){for(;!(b=(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))^a);)if(!((n=n+4|0)>>>0<_>>>0))break A;n=(Rf(b)>>>3|0)+n|0;break l}if(!(v>>>0<=n>>>0)){for(;;){if(gf[0|n]!=(255&a))break l;if(a=a>>>8|0,(0|v)==(0|(n=n+1|0)))break}n=v}}u=(u-R|0)+n|0}a=m}for(wf[65596+Z>>2]=y,b=(s=a)+4|0,a=g;b>>>0<=(n=a)>>>0&&(0|y)==(gf[0|(a=n-4|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24)););l:if(!(n>>>0<=s>>>0)&&(b=D,(0|H)==gf[0|(a=n-1|0)]))for(;;){if((n=a)>>>0<=s>>>0){n=s;break l}if(b=b-1|0,gf[0|(a=n-1|0)]!=gf[0|b])break}if(!(L>>>0<=l>>>0|Y|(0|R)!=(g-(s=g-n|0)|0))){for(g=Cf(y,0-s<<3),b=m+4|0,l=(wf[65596+Z>>2]=g)>>>24|0,a=d;b>>>0<=(n=a)>>>0&&(0|g)==(gf[0|(a=n-4|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24)););l:if(!(n>>>0<=m>>>0)&&(b=E,(0|l)==gf[0|(a=n-1|0)]))for(;;){if((n=a)>>>0<=m>>>0)break l;if(b=b-1|0,gf[0|(a=n-1|0)]!=gf[0|b])break}s=(s+d|0)-n|0}if((a=(P-(m=q>>>0<(a=P-s|0)>>>0?a:q)|0)+u|0)>>>0<j>>>0|j>>>0<u>>>0)if(n=2,L+(-1^m)>>>(l=0)<3)s=2,u=L;else{if((b=a>>>0<j>>>0?a:j)>>>0<=c>>>0)s=w,b=c;else if(65535<(F-(s=m+B|0)|0))break u;if(m>>>0<(a=Sf[131072+(((65535&m)<<1)+f|0)>>1])>>>0){w=s,c=b;break u}u=m-a|0,w=s,s=2,c=b}else u=L+(-1^(a=(P-j|0)+u|0))>>>0<3?L:a,l=0,s=n=2}if(3!=(0|n))continue}break}u:if(!(!C|65534<U-q>>>0||(l=wf[af+hf>>2],m=wf[af+262148>>2],65535<U-(s=(q+l|0)-(u=wf[af+262144>>2]-m|0)|0)>>>0)))for(;;){if(C=C-1|0,(0|y)==(gf[0|(a=l+m|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24))){n=a+4|0,d=s+B|0;c:{if(!((R=(h=v>>>0<(b=(u-l|0)+F|0)>>>0?v:b)-3|0)>>>0<=(b=a=p)>>>0)){if(a=(gf[0|p]|gf[p+1|0]<<8|(gf[p+2|0]<<16|gf[p+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))){a=Rf(a)>>>3|0;break c}n=n+4|0,b=A}if((a=b)>>>0<R>>>0)for(;;){if(b=(gf[0|a]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))){a=((Rf(b)>>>3|0)+a|0)-p|0;break c}if(n=n+4|0,!((a=a+4|0)>>>0<R>>>0))break}(gf[0|n]|gf[n+1|0]<<8)!=(gf[0|a]|gf[a+1|0]<<8)|h-1>>>0<=a>>>0||(n=n+2|0,a=a+2|0),a>>>0<h>>>0&&(a=gf[0|n]==gf[0|a]?a+1|0:a),a=a-p|0}w=(b=(0|c)<(0|(a=a+4|0)))?d:w,c=b?a:c}if(!C)break u;if(l=l-(a=Sf[131072+(af+((65535&l)<<1)|0)>>1])|0,!(U-(s=s-a|0)>>>0<65536))break}if((0|c)<4)break t;h=G&&c-19>>>0<18?18:c,b=F-w|0;break n}if(U=F-B|0,w=wf[f+262160>>2],s=U>>>0<w+65536>>>0,u=U-65535|0,L=wf[f+262156>>2],R=L+B|0,y=gf[0|F]|gf[F+1|0]<<8|(gf[F+2|0]<<16|gf[F+3|0]<<24),!(U>>>0<=c>>>0)&&((F-B|0)-(n=c)&1&&(b=(Bf(gf[0|(a=c+B|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24),-1640531535)>>>15&131068)+f|0,a=c-wf[b>>2]|0,vf[131072+(((65535&c)<<1)+f|0)>>1]=a>>>0<65535?a:65535,n=(wf[b>>2]=c)+1|0),(F+z|0)!=(0|c)))for(;b=(Bf(gf[0|(a=n+B|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24),-1640531535)>>>15&131068)+f|0,a=n-wf[b>>2]|0,vf[ff+((65535&n)<<1)>>1]=a>>>0<65535?a:65535,wf[b>>2]=n,b=(Bf(gf[0|(a=(c=n+1|0)+B|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24),-1640531535)>>>15&131068)+f|0,a=c-wf[b>>2]|0,vf[ff+((65535&c)<<1)>>1]=a>>>0<65535?a:65535,wf[b>>2]=c,(n=n+2|0)>>>0<U>>>0;);for(ef=s?w:u,wf[f+262164>>2]=U,H=(65535&y)==(y>>>16|0)&(0|(P=y>>>24|0))==(255&y),tf=(w=0)-F|0,C=L+X|0,d=R+4|0,A=F+8|0,p=F+4|0,Q=F-1|0,N=Bf(gf[0|F]|gf[F+1|0]<<8|(gf[F+2|0]<<16|gf[F+3|0]<<24),-1640531535)>>>17<<2,u=wf[N+f>>2],m=t,h=hf=I-J|(l=s=j=0);;){s:if(!(!m|u>>>0<ef>>>0)){u:if(!(U-u>>>(c=0)<8&&G)){c:{l:{A:{h:{if(L>>>0<=u>>>0){if((0|(b=gf[0|(a=h+Q|0)]|gf[a+1|0]<<8))!=(gf[0|(a=((g=u+B|0)+h|0)-1|0)]|gf[a+1|0]<<8)|(0|y)!=(gf[0|g]|gf[g+1|0]<<8|(gf[g+2|0]<<16|gf[g+3|0]<<24)))break u;if(n=g+4|0,(b=_)>>>0<=p>>>0)a=p;else{if(a=(gf[0|p]|gf[p+1|0]<<8|(gf[p+2|0]<<16|gf[p+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24)))break h;n=n+4|0,a=A}if(a>>>0<b>>>0)for(;;){if(b=(gf[0|a]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))){n=((Rf(b)>>>3|0)+a|0)-p|0;break c}if(n=n+4|0,!((a=a+4|0)>>>0<_>>>0))break}(gf[0|n]|gf[n+1|0]<<8)!=(gf[0|a]|gf[a+1|0]<<8)|$>>>0<=a>>>0||(n=n+2|0,a=a+2|0),a>>>0<v>>>0&&(a=gf[0|n]==gf[0|a]?a+1|0:a),n=a-p|0;break c}if((0|y)!=(gf[0|(a=u+X|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24)))break u;if(n=a+4|0,!((g=(b=v>>>0<(Y=(L-u|0)+F|0)>>>0?v:Y)-3|0)>>>0<=(c=a=p)>>>0)){if(a=(gf[0|p]|gf[p+1|0]<<8|(gf[p+2|0]<<16|gf[p+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24)))break A;n=n+4|0,c=A}if((a=c)>>>0<g>>>0)for(;;){if(c=(gf[0|a]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))){a=((Rf(c)>>>3|0)+a|0)-p|0;break l}if(n=n+4|0,!((a=a+4|0)>>>0<g>>>0))break}(gf[0|n]|gf[n+1|0]<<8)!=(gf[0|a]|gf[a+1|0]<<8)|b-1>>>0<=a>>>0||(n=n+2|0,a=a+2|0),a>>>0<b>>>0&&(a=gf[0|n]==gf[0|a]?a+1|0:a),a=a-p|0;break l}n=Rf(a)>>>3|0;break c}a=Rf(a)>>>3|0}if(W=u+B|0,!((0|b)!=(F+(c=a+4|0)|0)|v>>>0<=Y>>>0)){a=R;l:{A:{if((n=b)>>>0<_>>>0){if(a=(gf[0|b]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24))^(gf[0|R]|gf[R+1|0]<<8|(gf[R+2|0]<<16|gf[R+3|0]<<24)))break A;n=b+4|0,a=d}if(n>>>0<_>>>0)for(;;){if(g=(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))^(gf[0|a]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24))){a=((Rf(g)>>>3|0)+n|0)-b|0;break l}if(a=a+4|0,!((n=n+4|0)>>>0<_>>>0))break}(gf[0|a]|gf[a+1|0]<<8)!=(gf[0|n]|gf[n+1|0]<<8)|$>>>0<=n>>>0||(n=n+2|0,a=a+2|0),n>>>0<v>>>0&&(n=gf[0|a]==gf[0|n]?n+1|0:n),a=n-b|0;break l}a=Rf(a)>>>3|0}c=a+c|0}w=(a=(0|h)<(0|c))?W:w,h=a?c:h;break u}h=(a=(0|h)<(0|(c=n+4|0)))?c:h,w=a?g:w}m=m-1|0;u:{if(!((0|c)!=(0|h)|(0|c)<4|U>>>0<u+h>>>0)){for(Y=c-3|0,n=0,b=16,a=1;a=(q=a>>>0<(g=Sf[131072+(((n+u&65535)<<1)+f|0)>>1])>>>0)?g:a,l=q?n:l,g=b>>4,b=q?16:b+1|0,(0|(n=n+g|0))<(0|Y););if(n=(b=u>>>0<a>>>0)?0:a,u=u-((a=1<a>>>0)?n:0)|0,a){n=b?3:2,h=c;break u}}c:{l:{A:if(!(1!=Sf[131072+(((65535&u)<<1)+f|0)>>1]|l)){if(!s){if(s=1,!H)break A;h:{p:if(!(_>>>0<=(n=p)>>>0)){for(;!(a=y^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24)));)if(!((n=n+4|0)>>>0<_>>>0))break p;n=(Rf(a)>>>3|0)+n|0;break h}if(a=y,!(v>>>0<=n>>>0)){for(;;){if(gf[0|n]!=(255&a))break h;if(a=a>>>8|0,(0|v)==(0|(n=n+1|0)))break}n=v}}j=n+tf|0,s=2}if(q=u-1|0,!(2!=(0|s)|q>>>0<ef>>>0||(s=2,L-u>>>0<3||(0|y)!=(gf[0|(g=q+((Y=q>>>0<L>>>0)?X:B)|0)]|gf[g+1|0]<<8|(gf[g+2|0]<<16|gf[g+3|0]<<24))))){if(l=wf[f+262160>>2],(u=(b=Y?C:v)-3|0)>>>0<=(n=s=g+4|0)>>>0)break l;for(;!(a=y^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24)));)if(!((n=n+4|0)>>>0<u>>>0))break l;n=(Rf(a)>>>3|0)+n|0;break c}}u=u-Sf[131072+(((u+l&65535)<<1)+f|0)>>1]|0,n=0;break u}if(a=y,!(b>>>0<=n>>>0)){for(;;){if(gf[0|n]!=(255&a))break c;if(a=a>>>8|0,(0|b)==(0|(n=n+1|0)))break}n=b}}if(c=l+X|0,u=4+(n-s|0)|0,L>>>0<=q>>>0)a=R;else{if((0|b)==(u+g|0)){a=Cf(y,u<<3);c:{l:if(!(_>>>0<=(n=R)>>>0)){for(;!(b=(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))^a);)if(!((n=n+4|0)>>>0<_>>>0))break l;n=(Rf(b)>>>3|0)+n|0;break c}if(!(v>>>0<=n>>>0)){for(;;){if(gf[0|n]!=(255&a))break c;if(a=a>>>8|0,(0|v)==(0|(n=n+1|0)))break}n=v}}u=(u-R|0)+n|0}a=c}for(wf[65596+Z>>2]=y,b=(s=a)+4|0,a=g;b>>>0<=(n=a)>>>0&&(0|y)==(gf[0|(a=n-4|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24)););c:if(!(n>>>0<=s>>>0)&&(b=O,(0|P)==gf[0|(a=n-1|0)]))for(;;){if((n=a)>>>0<=s>>>0){n=s;break c}if(b=b-1|0,gf[0|(a=n-1|0)]!=gf[0|b])break}if(!(L>>>0<=l>>>0|Y|(0|R)!=(g-(s=g-n|0)|0))){for(g=Cf(y,0-s<<3),b=c+4|0,l=(wf[65596+Z>>2]=g)>>>24|0,a=C;b>>>0<=(n=a)>>>0&&(0|g)==(gf[0|(a=n-4|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24)););c:if(!(n>>>0<=c>>>0)&&(b=of,(0|l)==gf[0|(a=n-1|0)]))for(;;){if((n=a)>>>0<=c>>>0)break c;if(b=b-1|0,gf[0|(a=n-1|0)]!=gf[0|b])break}s=(s+C|0)-n|0}if((a=(q-(c=ef>>>0<(a=q-s|0)>>>0?a:ef)|0)+u|0)>>>0<j>>>0|j>>>0<u>>>0)if(n=2,L+(-1^c)>>>(l=0)<3)s=2,u=L;else{if((b=a>>>0<j>>>0?a:j)>>>0<=h>>>0)s=w,b=h;else if(65535<(F-(s=c+B|0)|0))break s;if(c>>>0<(a=Sf[131072+(((65535&c)<<1)+f|0)>>1])>>>0){w=s,h=b;break s}u=c-a|0,w=s,s=2,h=b}else u=L+(-1^(a=(q-j|0)+u|0))>>>0<3?L:a,l=0,s=n=2}if(3!=(0|n))continue}break}s:if(!(!m|65534<U-ef>>>0||(l=wf[N+af>>2],c=wf[af+262148>>2],65535<U-(s=(ef+l|0)-(u=wf[af+262144>>2]-c|0)|0)>>>0)))for(;;){if(m=m-1|0,(0|y)==(gf[0|(a=c+l|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24))){n=a+4|0;u:{if(!((R=(d=v>>>0<(b=(u-l|0)+F|0)>>>0?v:b)-3|0)>>>0<=(b=a=p)>>>0)){if(a=(gf[0|p]|gf[p+1|0]<<8|(gf[p+2|0]<<16|gf[p+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))){a=Rf(a)>>>3|0;break u}n=n+4|0,b=A}if((a=b)>>>0<R>>>0)for(;;){if(b=(gf[0|a]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))){a=((Rf(b)>>>3|0)+a|0)-p|0;break u}if(n=n+4|0,!((a=a+4|0)>>>0<R>>>0))break}(gf[0|n]|gf[n+1|0]<<8)!=(gf[0|a]|gf[a+1|0]<<8)|d-1>>>0<=a>>>0||(n=n+2|0,a=a+2|0),a>>>0<d>>>0&&(a=gf[0|n]==gf[0|a]?a+1|0:a),a=a-p|0}h=(b=(0|h)<(0|(a=a+4|0)))?a:h,w=b?s+B|0:w}if(!m)break s;if(l=l-(a=Sf[131072+(af+((65535&l)<<1)|0)>>1])|0,!(U-(s=s-a|0)>>>0<65536))break}if((0|h)<=(0|hf))break t;if(b=F-w|0,!G|18<=h-19>>>0){if(!h)break t}else h=18}n:{s:{if(!(4095<(h+J|0)|T>>>0<h>>>0)){if(w=(p=wf[uf+12>>2])-15|0,!(s=14<(0|p))){if(n=p+1|0,(0|(a=(u=bf-p|0)+(14==(0|p)?16:n)|0))<(0|pf)&&(wf[(c=(J+1<<4)+Z|0)+12>>2]=n,wf[c+4>>2]=0,wf[c+8>>2]=1,wf[c>>2]=a),a=n=p+2|0,12<(0|p)&&(a=3+(p+(((p<<16)-851968>>16)/255<<16>>16)|0)|0),(0|(a=a+u|0))<wf[(c=(J+2<<4)+Z|0)>>2]&&(wf[c+12>>2]=n,wf[c+4>>2]=0,wf[c+8>>2]=1,wf[c>>2]=a),a=n=p+3|0,12<=(0|p)&&(a=4+(p+(((p<<16)-786432>>16)/255<<16>>16)|0)|0),(0|(a=a+u|0))>=wf[(c=(J+3<<4)+Z|0)>>2])break n;wf[c+12>>2]=n,wf[c+4>>2]=0,wf[c+8>>2]=1,wf[c>>2]=a;break n}if((0|(a=(c=bf+((-1^p)-((w>>>0)/255|0)|0)|0)+((u=p+2|0)+((p-14|0)/255|0)|0)|0))<(0|pf)&&(wf[(n=(J+1<<4)+Z|0)+12>>2]=p+1,wf[n+4>>2]=0,wf[n+8>>2]=1,wf[n>>2]=a),(0|(a=c+((n=p+3|0)+((p-13|0)/255|0)|0)|0))<wf[(m=(J+2<<4)+Z|0)>>2]&&(wf[m+12>>2]=u,wf[m+4>>2]=0,wf[m+8>>2]=1,wf[m>>2]=a),(0|(a=4+(c+(p+((p-12|0)/255|0)|0)|0)|0))<wf[(u=(J+3<<4)+Z|0)>>2])break s;break n}I=J+1|0;break k}wf[u+12>>2]=n,wf[u+4>>2]=0,wf[u+8>>2]=1,wf[u>>2]=a}if(!((0|h)<4))if(a=(w>>>0)/255|0,1!=wf[8+((J<<(n=4))+Z|0)>>2])for(;(0|(a=bf+(19<=n>>>0?4+((n-19|0)/255|0)|0:3)|0))>(mf+wf[((s=n+J|0)<<4)+Z>>2]|0)&&(0|s)<=(I+3|0)||(wf[(u=(s<<4)+Z|0)+12>>2]=0,wf[u+4>>2]=b,wf[u+8>>2]=n,wf[u>>2]=a,I=(0|n)==(0|h)&&(0|I)<(0|s)?s:I),a=(0|n)==(0|h),n=n+1|0,!a;);else if(w=(a=s?1+(a+p|0)|0:p)+4|0,u=a+3|0,(0|p)<(0|J))for(c=(J-p<<4)+Z|0;(0|(a=(a=19<=n>>>0?w+((n-19|0)/255|0)|0:u)+(s=wf[c>>2])|0))>(mf+wf[((m=n+J|0)<<4)+Z>>2]|0)&&(0|m)<=(I+3|0)||(wf[(s=(m<<4)+Z|0)+12>>2]=p,wf[s+4>>2]=b,wf[s+8>>2]=n,wf[s>>2]=a,I=(0|n)==(0|h)&&(0|I)<(0|m)?m:I),a=(0|n)==(0|h),n=n+1|0,!a;);else for(;s=u,(0|(s=19<=n>>>0?w+((n-19|0)/255|0)|0:s))>(mf+wf[((c=n+J|0)<<4)+Z>>2]|0)&&(0|c)<=(I+3|0)||(wf[(a=(c<<4)+Z|0)+12>>2]=p,wf[a+4>>2]=b,wf[a+8>>2]=n,wf[a>>2]=s,I=(0|n)==(0|h)&&(0|I)<(0|c)?c:I),a=(0|n)==(0|h),n=n+1|0,!a;);wf[(b=(I<<4)+Z|0)+28>>2]=1,wf[b+20>>2]=0,wf[b+24>>2]=1,wf[b+36>>2]=0,wf[b+40>>2]=1,wf[b+44>>2]=2,wf[b+60>>2]=3,wf[b+52>>2]=0,wf[b+56>>2]=1,a=wf[b>>2],wf[b+16>>2]=a+1,wf[b+32>>2]=a+2,wf[b+48>>2]=a+3}if(Af>>>0<(F=x+kf|0)>>>0)break o;if(c=U,!((0|(J=kf))<(0|I)))break}J=I-(h=wf[(a=(I<<4)+Z|0)+8>>2])|0,b=wf[a+4>>2]}for(;n=wf[(u=(J<<4)+Z|0)+8>>2],wf[u+8>>2]=h,a=wf[u+4>>2],wf[u+4>>2]=b,u=(0|n)<=(0|J),J=J-n|0,h=n,b=a,u;);if(!((0|I)<1)){if(s=0,!k)for(;;){if(1==(0|(m=wf[(a=(s<<4)+Z|0)+8>>2])))s=s+1|0,x=x+1|0;else{for(l=S+1|0,u=wf[a+4>>2],(w=x-M|0)>>>0<=14?df[0|S]=w<<4:(df[0|S]=240,255<=(n=w-15|0)>>>0&&(If(l,255,(b=((a=w-270|0)>>>0)/255|0)+1|0),l=2+(b+S|0)|0,n=a+Bf(b,-255)|0),df[0|l]=n,l=l+1|0),s=s+m|0,c=l+w|0,n=l;b=gf[M+4|0]|gf[M+5|0]<<8|(gf[M+6|0]<<16|gf[M+7|0]<<24),a=gf[0|M]|gf[M+1|0]<<8|(gf[M+2|0]<<16|gf[M+3|0]<<24),df[0|n]=a,df[n+1|0]=a>>>8,df[n+2|0]=a>>>16,df[n+3|0]=a>>>24,df[n+4|0]=b,df[n+5|0]=b>>>8,df[n+6|0]=b>>>16,df[n+7|0]=b>>>24,M=M+8|0,(n=n+8|0)>>>0<c>>>0;);(df[0|c]=u,df[c+1|0]=u>>>8,n=c+2|0,b=gf[0|S],(a=m-4|0)>>>0<=14)?(df[0|S]=a+b,M=x=m+x|0,S=n):(df[0|S]=b+15,510<=(a=m-19|0)>>>0&&(If(u=n,255,(b=(a=((n=m-529|0)>>>0)/510|0)<<1)+2|0),a=n+Bf(a,-510)|0,n=4+((b+w|0)+l|0)|0),255<=a>>>0&&(df[0|n]=255,n=n+1|0,a=a-255|0),df[0|n]=a,S=n+1|0,M=x=m+x|0)}if(!((0|s)<(0|I)))break b}for(;;){if(1!=(0|(p=wf[(a=(s<<4)+Z|0)+8>>2]))){if(lf>>>0<9+(((((U=x-M|0)>>>0)/255|0)+S|0)+U|0)>>>0)break a;for(u=S+1|0,c=wf[a+4>>2],15<=U>>>0?(df[0|S]=240,255<=(n=U-15|0)>>>0&&(If(u,255,(b=((a=U-270|0)>>>0)/255|0)+1|0),u=2+(b+S|0)|0,n=a+Bf(b,-255)|0),df[0|u]=n,u=u+1|0):df[0|S]=U<<4,s=s+p|0,m=u+U|0,n=M,a=u;w=gf[n+4|0]|gf[n+5|0]<<8|(gf[n+6|0]<<16|gf[n+7|0]<<24),b=gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24),df[0|a]=b,df[a+1|0]=b>>>8,df[a+2|0]=b>>>16,df[a+3|0]=b>>>24,df[a+4|0]=w,df[a+5|0]=w>>>8,df[a+6|0]=w>>>16,df[a+7|0]=w>>>24,n=n+8|0,(a=a+8|0)>>>0<m>>>0;);if(df[0|m]=c,df[m+1|0]=c>>>8,lf>>>0<6+((n=m+2|0)+(((b=p-4|0)>>>0)/255|0)|0)>>>0)break a;a=gf[0|S],15<=b>>>0?(df[0|S]=a+15,510<=(a=p-19|0)>>>0&&(If(c=n,255,(b=(a=((n=p-529|0)>>>0)/510|0)<<1)+2|0),a=n+Bf(a,-510)|0,n=4+((b+U|0)+u|0)|0),255<=a>>>0&&(df[0|n]=255,n=n+1|0,a=a-255|0),df[0|n]=a,n=n+1|0):df[0|S]=a+b,S=n,M=x=p+x|0}else s=s+1|0,x=x+1|0;if(!((0|s)<(0|I)))break}}}if(Af>>>0<x>>>0)break i;B=wf[f+262148>>2];continue}break}if(2!=((n=0)|k))break e}if(n=K-M|0,a=(n+240>>>0)/255|0,k&&!((b=1+((a+n|0)+S|0)|0)>>>0<=(a=yf?lf+5|0:nf)>>>0)){if(1==((n=0)|k))break e;n=(a=a+(-1^S)|0)-((a+240>>>0)/255|0)|0}k=n+M|0,15<=n>>>0?(df[0|S]=240,a=S+1|0,(b=n-15|0)>>>0<255?df[0|(S=a)]=b:(If(u=a,255,(b=((a=n-270|0)>>>0)/255|0)+1|0),df[0|(S=2+(b+S|0)|0)]=a+Bf(b,-255))):df[0|S]=n<<4,a=Ef(S+1|0,M,n),wf[i>>2]=k-r,n=(a+n|0)-e|0}if(0<(0|n))break f}df[f+262171|0]=1}}else{if(p=Ef(f,wf[f+262172>>2],262176),w=wf[p+262144>>2],s=wf[p+262156>>2],m=wf[p+262148>>2],!(w>>>0<4+(s+m|0)>>>0||(c=(w-m|0)-3|0)>>>0<=(f=wf[p+262164>>2])>>>0)){for(;u=p+(Bf(gf[0|(n=f+m|0)]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24),-1640531535)>>>15&131068)|0,n=f-wf[u>>2]|0,vf[131072+(p+((65535&f)<<1)|0)>>1]=n>>>0<65535?n:65535,(f=(wf[u>>2]=f)+1|0)>>>0<c>>>0;);s=wf[p+262156>>2]}wf[p+262160>>2]=s,wf[p+262152>>2]=m,wf[p+262172>>2]=0,wf[p+262144>>2]=r,vf[p+262168>>1]=b,f=w-m|0,wf[p+262156>>2]=f,wf[p+262164>>2]=f,wf[p+262148>>2]=r-f,n=Zf(p,r,e,i,a,b,k)}return _f=65600+Z|0,n}function Zf(f,r,e,i,a,b,k){var o,t,n=0,s=0,u=0,c=0,l=0,A=0,h=0,p=0,m=0,y=0,d=0,v=0,w=0,g=0,S=0,B=0,_=0,Z=0,E=0,I=0,C=0,R=0,U=0,J=0,F=0,M=0,x=0,O=0,V=0,W=0,L=0,j=0,T=0,N=0,X=0,D=0,P=0,Y=0,z=0,G=0,Q=0,H=0,q=0,K=0,$=0,ff=0,rf=0,ef=0,af=0,bf=0,kf=0,of=0,tf=0,nf=0,sf=0,uf=0,cf=0,lf=0,Af=0,hf=0,pf=0;_f=S=_f-65600|0;f:if(!(2==(0|k)&&(0|a)<=0||2113929216<(A=wf[i>>2])>>>0)){wf[f+262144>>2]=A+wf[f+262144>>2],b=Bf(n=(0|(u=(0|b)<1?9:b))<12?u:12,12),t=wf[b+1764>>2];r:{e:if(n>>>0<=9){cf=e+a|(wf[i>>2]=0),o=(pf=2==(0|k))?cf-5|0:cf,sf=r+A|0,g=r,p=e;i:if(!((0|A)<13||(kf=sf-12|0)>>>0<r>>>0))for(uf=6656>>>n&1,lf=(y=sf-5|0)-1|0,D=y-3|0,G=Y=V=C=hf=Af=3|S,rf=f+131072|0,g=M=r;;){if(a=wf[f+262160>>2],q=wf[f+262148>>2],A=M-q|0,of=A>>>0<a+65536>>>0?a:A-65535|0,z=wf[f+262156>>2],Z=gf[0|M]|gf[M+1|0]<<8|(gf[M+2|0]<<16|gf[M+3|0]<<24),R=wf[f+262152>>2],n=wf[f+262164>>2],!(A>>>0<=n>>>0)&&(u=(-1^n)+M|0,(M-n|0)-q&1&&(b=(Bf(gf[0|(a=n+q|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24),-1640531535)>>>15&131068)+f|0,a=n-wf[b>>2]|0,vf[131072+(((65535&n)<<1)+f|0)>>1]=a>>>0<65535?a:65535,n=(wf[b>>2]=n)+1|0),(0|u)!=(0|q)))for(;b=(Bf(gf[0|(a=n+q|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24),-1640531535)>>>15&131068)+f|0,a=n-wf[b>>2]|0,vf[rf+((65535&n)<<1)>>1]=a>>>0<65535?a:65535,wf[b>>2]=n,b=(Bf(gf[0|(a=(u=n+1|0)+q|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24),-1640531535)>>>15&131068)+f|0,a=u-wf[b>>2]|0,vf[rf+((65535&u)<<1)>>1]=a>>>0<65535?a:65535,wf[b>>2]=u,(n=n+2|0)>>>0<A>>>0;);wf[f+262164>>2]=A;a:{b:{if(of>>>0<=(u=wf[(Bf(gf[0|M]|gf[M+1|0]<<8|(gf[M+2|0]<<16|gf[M+3|0]<<24),-1640531535)>>>15&131068)+f>>2])>>>0){for(B=(65535&Z)==(Z>>>16|0)&(0|(F=Z>>>24|0))==(255&Z),x=R+z|0,j=(c=z+q|0)+4|0,m=M+8|0,T=M-1|0,af=4-(v=M+4|0)|0,A=3,d=t,U=X=0;;){k:{o:{t:{n:{s:{if(z>>>0<=u>>>0){if((0|(b=gf[0|(a=A+T|0)]|gf[a+1|0]<<8))!=(gf[0|(a=((s=u+q|0)+A|0)-1|0)]|gf[a+1|0]<<8)|(0|Z)!=(gf[0|s]|gf[s+1|0]<<8|(gf[s+2|0]<<16|gf[s+3|0]<<24)))break k;if(n=s+4|0,(a=D)>>>0<=v>>>0)b=v;else{if(b=(gf[0|v]|gf[v+1|0]<<8|(gf[v+2|0]<<16|gf[v+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24)))break s;n=n+4|0,b=m}if(b>>>0<a>>>0)for(;;){if(a=(gf[0|b]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))){n=((Rf(a)>>>3|0)+b|0)-v|0;break o}if(n=n+4|0,!((b=b+4|0)>>>0<D>>>0))break}(gf[0|n]|gf[n+1|0]<<8)!=(gf[0|b]|gf[b+1|0]<<8)|lf>>>0<=b>>>0||(n=n+2|0,b=b+2|0),b>>>0<y>>>0&&(b=gf[0|n]==gf[0|b]?b+1|0:b),n=b-v|0;break o}if((0|Z)!=(gf[0|(a=u+R|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24)))break k;if(n=a+4|0,!((l=(a=y>>>0<(W=(z-u|0)+M|0)>>>0?y:W)-3|0)>>>0<=(s=b=v)>>>0)){if(b=(gf[0|v]|gf[v+1|0]<<8|(gf[v+2|0]<<16|gf[v+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24)))break n;n=n+4|0,s=m}if((b=s)>>>0<l>>>0)for(;;){if(s=(gf[0|b]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))){b=((Rf(s)>>>3|0)+b|0)-v|0;break t}if(n=n+4|0,!((b=b+4|0)>>>0<l>>>0))break}(gf[0|n]|gf[n+1|0]<<8)!=(gf[0|b]|gf[b+1|0]<<8)|a-1>>>0<=b>>>0||(n=n+2|0,b=b+2|0),b>>>0<a>>>0&&(b=gf[0|n]==gf[0|b]?b+1|0:b),b=b-v|0;break t}n=Rf(b)>>>3|0;break o}b=Rf(b)>>>3|0}if(O=u+q|0,!((0|a)!=(M+(s=b+4|0)|0)|y>>>0<=W>>>0)){b=c;t:{n:{if((n=a)>>>0<D>>>0){if(b=(gf[0|a]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24))^(gf[0|c]|gf[c+1|0]<<8|(gf[c+2|0]<<16|gf[c+3|0]<<24)))break n;n=a+4|0,b=j}if(n>>>0<D>>>0)for(;;){if(l=(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))^(gf[0|b]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24))){a=((Rf(l)>>>3|0)+n|0)-a|0;break t}if(b=b+4|0,!((n=n+4|0)>>>0<D>>>0))break}(gf[0|b]|gf[b+1|0]<<8)!=(gf[0|n]|gf[n+1|0]<<8)|lf>>>0<=n>>>0||(n=n+2|0,b=b+2|0),n>>>0<y>>>0&&(n=gf[0|b]==gf[0|n]?n+1|0:n),a=n-a|0;break t}a=Rf(b)>>>3|0}s=a+s|0}L=(a=(0|A)<(0|s))?O:L,A=a?s:A;break k}A=(b=(0|A)<(0|(a=n+4|0)))?a:A,L=b?s:L}k:{o:{t:{n:{s:if(!(!uf|1!=(0|(s=Sf[131072+(((65535&u)<<1)+f|0)>>1])))){if(!X){if(X=1,!B)break s;u:{c:if(!(D>>>0<=(n=v)>>>0)){for(;!(a=Z^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24)));)if(!((n=n+4|0)>>>0<D>>>0))break c;n=(Rf(a)>>>3|0)+n|0;break u}if(b=Z,!(y>>>0<=n>>>0)){for(;;){if(gf[0|n]!=(255&b))break u;if(b=b>>>8|0,(0|y)==(0|(n=n+1|0)))break}n=y}}U=n+af|0,X=2}if(bf=u-1|0,!(2!=(0|X)|bf>>>0<of>>>0||(X=2,z-u>>>0<3||(0|Z)!=(gf[0|(l=bf+((N=bf>>>0<z>>>0)?R:q)|0)]|gf[l+1|0]<<8|(gf[l+2|0]<<16|gf[l+3|0]<<24))))){if(O=wf[f+262160>>2],(u=(a=N?x:y)-3|0)>>>0<=(n=W=l+4|0)>>>0)break n;for(;!(b=Z^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24)));)if(!((n=n+4|0)>>>0<u>>>0))break n;n=(Rf(b)>>>3|0)+n|0;break t}}b=u-s|0;break o}if(b=Z,!(a>>>0<=n>>>0)){for(;;){if(gf[0|n]!=(255&b))break t;if(b=b>>>8|0,(0|a)==(0|(n=n+1|0)))break}n=a}}if(s=R+O|0,W=4+(n-W|0)|0,z>>>0<=bf>>>0)a=c;else{if((0|a)==(l+W|0)){b=Cf(Z,W<<3);t:{n:if(!(D>>>0<=(n=c)>>>0)){for(;!(a=(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))^b);)if(!((n=n+4|0)>>>0<D>>>0))break n;n=(Rf(a)>>>3|0)+n|0;break t}if(!(y>>>0<=n>>>0)){for(;;){if(gf[0|n]!=(255&b))break t;if(b=b>>>8|0,(0|y)==(0|(n=n+1|0)))break}n=y}}W=(W-c|0)+n|0}a=s}for(wf[S>>2]=Z,u=a+4|0,b=l;u>>>0<=(n=b)>>>0&&(0|Z)==(gf[0|(b=n-4|0)]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24)););t:if(!(n>>>0<=a>>>0)&&(u=G,(0|F)==gf[0|(b=n-1|0)]))for(;;){if((n=b)>>>0<=a>>>0){n=a;break t}if(u=u-1|0,gf[0|(b=n-1|0)]!=gf[0|u])break}if(!(z>>>0<=O>>>0|N|(0|c)!=(l-(X=l-n|0)|0))){for(l=Cf(Z,0-X<<3),u=s+4|0,a=(wf[S>>2]=l)>>>24|0,b=x;u>>>0<=(n=b)>>>0&&(0|l)==(gf[0|(b=n-4|0)]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24)););t:if(!(n>>>0<=s>>>0)&&(u=Y,(0|a)==gf[0|(b=n-1|0)]))for(;;){if((n=b)>>>0<=s>>>0)break t;if(u=u-1|0,gf[0|(b=n-1|0)]!=gf[0|u])break}X=(x+X|0)-n|0}if(U>>>0<W>>>0|(a=(bf-(n=of>>>0<(a=bf-X|0)>>>0?a:of)|0)+W|0)>>>0<U>>>0){if(X=2,!((b=u=z)+(-1^n)>>>0<3)){if((b=a>>>0<U>>>0?a:U)>>>0<=A>>>0)l=L,b=A;else if(65535<(M-(l=n+q|0)|0))break k;if(n>>>0<(a=Sf[131072+(((65535&n)<<1)+f|0)>>1])>>>0){L=l,A=b;break k}L=l,A=b,b=n-a|0}}else X=2,b=z+(-1^(a=(bf-U|0)+W|0))>>>0<3?z:a}if(u=b,d=d-1|0,d&&of>>>0<=u>>>0)continue}break}if(3<(0|A))break b}M=M+1|0;break a}x=p,z=u=M,B=a=L,v=A;b:{k:{o:{t:for(;;){L=a;n:{if(!(kf>>>0<(M=u+(af=A)|0)>>>0)){if(a=wf[f+262160>>2],K=(A=(R=M-2|0)-(P=wf[f+262148>>2])|0)>>>0<a+65536>>>0?a:A-65535|0,E=wf[f+262156>>2],T=gf[0|R]|gf[R+1|0]<<8|(gf[R+2|0]<<16|gf[R+3|0]<<24),ff=wf[f+262152>>2],(n=wf[f+262164>>2])>>>0<A>>>0)for(;b=(Bf(gf[0|(a=n+P|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24),-1640531535)>>>15&131068)+f|0,a=n-wf[b>>2]|0,vf[131072+(((65535&n)<<1)+f|0)>>1]=a>>>0<65535?a:65535,(n=(wf[b>>2]=n)+1|0)>>>0<A>>>0;);if(wf[f+262164>>2]=A,!((c=wf[(Bf(gf[0|R]|gf[R+1|0]<<8|(gf[R+2|0]<<16|gf[R+3|0]<<24),-1640531535)>>>15&131068)+f>>2])>>>0<K>>>0)){for(X=(65535&T)==(T>>>16|0)&(0|(tf=T>>>24|0))==(255&T),O=E+ff|0,F=(l=E+P|0)+4|0,W=R+8|0,ef=u-R|0,q=0-(nf=R-u|0)|0,bf=u-1|0,of=4-(Z=R+4|0)|0,A=af,j=t,p=m=0,a=h;;){s:{u:{c:{l:{A:{if(E>>>0<=c>>>0){if((0|(n=gf[0|(b=A+bf|0)]|gf[b+1|0]<<8))!=(gf[0|(b=(((d=c+P|0)+q|0)+A|0)-1|0)]|gf[b+1|0]<<8)|(0|T)!=(gf[0|d]|gf[d+1|0]<<8|(gf[d+2|0]<<16|gf[d+3|0]<<24)))break s;h:if(nf)for(b=(h=(0|(b=l-d|0))<(0|ef)?ef:b)>>31&h,n=0;;){if((0|(s=n))<=(0|h)){s=b;break h}if(gf[R+(n=s-1|0)|0]!=gf[n+d|0])break}else s=0;if(n=d+4|0,(h=D)>>>0<=Z>>>0)b=Z;else{if(b=(gf[0|Z]|gf[Z+1|0]<<8|(gf[Z+2|0]<<16|gf[Z+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24)))break A;n=n+4|0,b=W}if(b>>>0<h>>>0)for(;;){if(h=(gf[0|b]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))){n=((Rf(h)>>>3|0)+b|0)-Z|0;break u}if(n=n+4|0,!((b=b+4|0)>>>0<D>>>0))break}(gf[0|n]|gf[n+1|0]<<8)!=(gf[0|b]|gf[b+1|0]<<8)|lf>>>0<=b>>>0||(n=n+2|0,b=b+2|0),b>>>0<y>>>0&&(b=gf[0|n]==gf[0|b]?b+1|0:b),n=b-Z|0;break u}if((0|T)!=(gf[0|($=c+ff|0)]|gf[$+1|0]<<8|(gf[$+2|0]<<16|gf[$+3|0]<<24)))break s;if(n=$+4|0,N=wf[f+262160>>2],!((d=(s=y>>>0<(Q=R+(E-c|0)|0)>>>0?y:Q)-3|0)>>>0<=(h=b=Z)>>>0)){if(b=(gf[0|Z]|gf[Z+1|0]<<8|(gf[Z+2|0]<<16|gf[Z+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24)))break l;n=n+4|0,h=W}if((b=h)>>>0<d>>>0)for(;;){if(h=(gf[0|b]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))){b=((Rf(h)>>>3|0)+b|0)-Z|0;break c}if(n=n+4|0,!((b=b+4|0)>>>0<d>>>0))break}(gf[0|n]|gf[n+1|0]<<8)!=(gf[0|b]|gf[b+1|0]<<8)|s-1>>>0<=b>>>0||(n=n+2|0,b=b+2|0),b>>>0<s>>>0&&(b=gf[0|n]==gf[0|b]?b+1|0:b),b=b-Z|0;break c}n=Rf(b)>>>3|0;break u}b=Rf(b)>>>3|0}if(!((0|s)!=(R+(d=b+4|0)|0)|y>>>0<=Q>>>0)){b=l;c:{l:{if((n=s)>>>0<D>>>0){if(b=(gf[0|s]|gf[s+1|0]<<8|(gf[s+2|0]<<16|gf[s+3|0]<<24))^(gf[0|l]|gf[l+1|0]<<8|(gf[l+2|0]<<16|gf[l+3|0]<<24)))break l;n=s+4|0,b=F}if(n>>>0<D>>>0)for(;;){if(h=(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))^(gf[0|b]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24))){b=((Rf(h)>>>3|0)+n|0)-s|0;break c}if(b=b+4|0,!((n=n+4|0)>>>0<D>>>0))break}(gf[0|b]|gf[b+1|0]<<8)!=(gf[0|n]|gf[n+1|0]<<8)|lf>>>0<=n>>>0||(n=n+2|0,b=b+2|0),n>>>0<y>>>0&&(n=gf[0|b]==gf[0|n]?n+1|0:n),b=n-s|0;break c}b=Rf(b)>>>3|0}d=b+d|0}c:if(nf)for(s=(h=(0|(b=(N+ff|0)-$|0))<(0|ef)?ef:b)>>31&h,n=0;;){if((0|(b=n))<=(0|h)){b=s;break c}if(gf[R+(n=b-1|0)|0]!=gf[n+$|0])break}else b=0;if((0|(n=d-b|0))<=(0|A))break s;_=b+R|0,a=(c+P|0)+b|0,A=n;break s}(0|(b=4+(n-s|0)|0))<=(0|A)||(_=s+R|0,a=s+d|0,A=b)}s:{u:{c:{l:{A:if(!(!uf|1!=(0|(s=Sf[131072+(((65535&c)<<1)+f|0)>>1])))){if(!m){if(m=1,!X)break A;h:{p:if(!(D>>>0<=(n=Z)>>>0)){for(;!(b=T^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24)));)if(!((n=n+4|0)>>>0<D>>>0))break p;n=(Rf(b)>>>3|0)+n|0;break h}if(b=T,!(y>>>0<=n>>>0)){for(;;){if(gf[0|n]!=(255&b))break h;if(b=b>>>8|0,(0|y)==(0|(n=n+1|0)))break}n=y}}m=2,p=n+of|0}if(H=c-1|0,!(2!=(0|m)|H>>>0<K>>>0||(m=2,E-c>>>0<3||(0|T)!=(gf[0|(N=H+(($=H>>>0<E>>>0)?ff:P)|0)]|gf[N+1|0]<<8|(gf[N+2|0]<<16|gf[N+3|0]<<24))))){if(Q=wf[f+262160>>2],(h=(s=$?O:y)-3|0)>>>0<=(n=c=N+4|0)>>>0)break l;for(;!(b=T^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24)));)if(!((n=n+4|0)>>>0<h>>>0))break l;n=(Rf(b)>>>3|0)+n|0;break c}}c=c-s|0;break u}if(b=T,!(s>>>0<=n>>>0)){for(;;){if(gf[0|n]!=(255&b))break c;if(b=b>>>8|0,(0|s)==(0|(n=n+1|0)))break}n=s}}if(h=Q+ff|0,m=4+(n-c|0)|0,E>>>0<=H>>>0)b=l;else{if((0|s)==(m+N|0)){b=Cf(T,m<<3);c:{l:if(!(D>>>0<=(n=l)>>>0)){for(;!(s=(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))^b);)if(!((n=n+4|0)>>>0<D>>>0))break l;n=(Rf(s)>>>3|0)+n|0;break c}if(!(y>>>0<=n>>>0)){for(;;){if(gf[0|n]!=(255&b))break c;if(b=b>>>8|0,(0|y)==(0|(n=n+1|0)))break}n=y}}m=(m-l|0)+n|0}b=h}for(wf[S>>2]=T,c=(s=b)+4|0,b=N;c>>>0<=(n=b)>>>0&&(0|T)==(gf[0|(b=n-4|0)]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24)););c:if(!(n>>>0<=s>>>0)&&(c=V,(0|tf)==gf[0|(b=n-1|0)]))for(;;){if((n=b)>>>0<=s>>>0){n=s;break c}if(c=c-1|0,gf[0|(b=n-1|0)]!=gf[0|c])break}if(!(E>>>0<=Q>>>0|$|(0|l)!=(N-(d=N-n|0)|0))){for(N=Cf(T,0-d<<3),c=h+4|0,s=(wf[S>>2]=N)>>>24|0,b=O;c>>>0<=(n=b)>>>0&&(0|N)==(gf[0|(b=n-4|0)]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24)););c:if(!(n>>>0<=h>>>0)&&(c=C,(0|s)==gf[0|(b=n-1|0)]))for(;;){if((n=b)>>>0<=h>>>0)break c;if(c=c-1|0,gf[0|(b=n-1|0)]!=gf[0|c])break}d=(d+O|0)-n|0}if((n=(H-(h=K>>>0<(b=H-d|0)>>>0?b:K)|0)+m|0)>>>0<p>>>0|p>>>0<m>>>0){if(c=(b=E+(-1^h)>>>0<3)?E:h,m=2,!(b|nf)){if((b=n>>>0<p>>>0?n:p)>>>0<=A>>>0)s=_,n=a,b=A;else if(65535<((s=R)-(n=h+P|0)|0))break s;if(h>>>0<(a=Sf[131072+(((65535&h)<<1)+f|0)>>1])>>>0){_=s,a=n,A=b;break s}c=h-a|0,_=s,a=n,A=b}}else c=E+(-1^(b=(H-p|0)+m|0))>>>0<3?E:b,m=2}if(j=j-1|0,j&&K>>>0<=c>>>0)continue}break}if((0|A)!=(0|af))break n;h=a}}if(o>>>0<9+(((((c=u-g|0)>>>0)/255|0)+x|0)+c|0)>>>0&&k)break o;for(l=x+1|0,15<=c>>>0?(df[0|x]=240,255<=(n=c-15|0)>>>0&&(If(l,255,(b=((a=c-270|0)>>>0)/255|0)+1|0),l=2+(b+x|0)|0,n=a+Bf(b,-255)|0),df[0|l]=n,l=l+1|0):df[0|x]=c<<4,s=c+l|0,n=g,b=l;A=gf[n+4|0]|gf[n+5|0]<<8|(gf[n+6|0]<<16|gf[n+7|0]<<24),a=gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24),df[0|b]=a,df[b+1|0]=a>>>8,df[b+2|0]=a>>>16,df[b+3|0]=a>>>24,df[b+4|0]=A,df[b+5|0]=A>>>8,df[b+6|0]=A>>>16,df[b+7|0]=A>>>24,n=n+8|0,(b=b+8|0)>>>0<s>>>0;);if(a=u-L|0,df[0|s]=a,df[s+1|0]=a>>>8,o>>>0<6+((p=s+2|0)+(((b=af-4|0)>>>0)/255|0)|0)>>>0&&k)break o;if(a=gf[0|x],15<=b>>>0){df[0|x]=a+15,510<=(n=af-19|0)>>>0&&(If(p,255,(a=(b=((n=af-529|0)>>>0)/510|0)<<1)+2|0),p=4+((a+c|0)+l|0)|0,n=n+Bf(b,-510)|0),255<=n>>>0&&(df[0|p]=255,p=p+1|0,n=n-255|0);break b}df[0|x]=a+b,g=M;break a}if(p=(b=_>>>0<u+v>>>0&z>>>0<u>>>0)?z:u,h=a,!(((u=_)-p|0)<3)){for(z=b?v:af,L=b?B:L,Z=g;;){X=(g=p+z|0)+3|0,$=(nf=(0|z)<18?z:18)+p|0;n:{s:{for(;;){17<(0|(b=u-p|0))||(0|(b=(p-u|0)+((u+A|0)-4>>>0<$>>>0?(b+A|0)-4|0:nf)|0))<1?v=A:(v=A-b|0,u=b+u|0,a=a+b|0),h=a;u:{if(!(kf>>>0<(M=v+(_=u)|0)>>>0)){if(a=wf[f+262160>>2],H=(u=(O=M-3|0)-(E=wf[f+262148>>2])|0)>>>0<a+65536>>>0?a:u-65535|0,U=wf[f+262156>>2],T=gf[0|O]|gf[O+1|0]<<8|(gf[O+2|0]<<16|gf[O+3|0]<<24),K=wf[f+262152>>2],(n=wf[f+262164>>2])>>>0<u>>>0)for(;b=(Bf(gf[0|(a=n+E|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24),-1640531535)>>>15&131068)+f|0,a=n-wf[b>>2]|0,vf[131072+(((65535&n)<<1)+f|0)>>1]=a>>>0<65535?a:65535,(n=(wf[b>>2]=n)+1|0)>>>0<u>>>0;);if(wf[f+262164>>2]=u,!((c=wf[(Bf(gf[0|O]|gf[O+1|0]<<8|(gf[O+2|0]<<16|gf[O+3|0]<<24),-1640531535)>>>15&131068)+f>>2])>>>0<H>>>0)){for(bf=(65535&T)==(T>>>16|0)&(0|(q=T>>>24|0))==(255&T),F=U+K|0,B=(l=E+U|0)+4|0,af=O+8|0,ff=_-O|0,of=0-(ef=O-_|0)|0,R=_-1|0,N=4-(W=O+4|0)|0,A=v,j=t,J=m=0,a=I,u=w;;){c:{l:{A:{h:{p:{if(U>>>0<=c>>>0){if((0|(n=gf[0|(b=A+R|0)]|gf[b+1|0]<<8))!=(gf[0|(b=(((I=c+E|0)+of|0)+A|0)-1|0)]|gf[b+1|0]<<8)|(0|T)!=(gf[0|I]|gf[I+1|0]<<8|(gf[I+2|0]<<16|gf[I+3|0]<<24)))break c;m:if(ef)for(b=(w=(0|(b=l-I|0))<(0|ff)?ff:b)>>31&w,n=0;;){if((0|(s=n))<=(0|w)){s=b;break m}if(gf[O+(n=s-1|0)|0]!=gf[n+I|0])break}else s=0;if(n=I+4|0,(w=D)>>>0<=W>>>0)b=W;else{if(b=(gf[0|W]|gf[W+1|0]<<8|(gf[W+2|0]<<16|gf[W+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24)))break p;n=n+4|0,b=af}if(b>>>0<w>>>0)for(;;){if(w=(gf[0|b]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))){n=((Rf(w)>>>3|0)+b|0)-W|0;break l}if(n=n+4|0,!((b=b+4|0)>>>0<D>>>0))break}(gf[0|n]|gf[n+1|0]<<8)!=(gf[0|b]|gf[b+1|0]<<8)|lf>>>0<=b>>>0||(n=n+2|0,b=b+2|0),b>>>0<y>>>0&&(b=gf[0|n]==gf[0|b]?b+1|0:b),n=b-W|0;break l}if((0|T)!=(gf[0|(Q=c+K|0)]|gf[Q+1|0]<<8|(gf[Q+2|0]<<16|gf[Q+3|0]<<24)))break c;if(n=Q+4|0,I=wf[f+262160>>2],!((d=(s=y>>>0<(tf=O+(U-c|0)|0)>>>0?y:tf)-3|0)>>>0<=(w=b=W)>>>0)){if(b=(gf[0|W]|gf[W+1|0]<<8|(gf[W+2|0]<<16|gf[W+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24)))break h;n=n+4|0,w=af}if((b=w)>>>0<d>>>0)for(;;){if(w=(gf[0|b]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))){b=((Rf(w)>>>3|0)+b|0)-W|0;break A}if(n=n+4|0,!((b=b+4|0)>>>0<d>>>0))break}(gf[0|n]|gf[n+1|0]<<8)!=(gf[0|b]|gf[b+1|0]<<8)|s-1>>>0<=b>>>0||(n=n+2|0,b=b+2|0),b>>>0<s>>>0&&(b=gf[0|n]==gf[0|b]?b+1|0:b),b=b-W|0;break A}n=Rf(b)>>>3|0;break l}b=Rf(b)>>>3|0}if(!((0|s)!=(O+(d=b+4|0)|0)|y>>>0<=tf>>>0)){b=l;A:{h:{if((n=s)>>>0<D>>>0){if(b=(gf[0|s]|gf[s+1|0]<<8|(gf[s+2|0]<<16|gf[s+3|0]<<24))^(gf[0|l]|gf[l+1|0]<<8|(gf[l+2|0]<<16|gf[l+3|0]<<24)))break h;n=s+4|0,b=B}if(n>>>0<D>>>0)for(;;){if(w=(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))^(gf[0|b]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24))){b=((Rf(w)>>>3|0)+n|0)-s|0;break A}if(b=b+4|0,!((n=n+4|0)>>>0<D>>>0))break}(gf[0|b]|gf[b+1|0]<<8)!=(gf[0|n]|gf[n+1|0]<<8)|lf>>>0<=n>>>0||(n=n+2|0,b=b+2|0),n>>>0<y>>>0&&(n=gf[0|b]==gf[0|n]?n+1|0:n),b=n-s|0;break A}b=Rf(b)>>>3|0}d=b+d|0}A:if(ef)for(s=(w=(0|(b=(I+K|0)-Q|0))<(0|ff)?ff:b)>>31&w,n=0;;){if((0|(b=n))<=(0|w)){b=s;break A}if(gf[O+(n=b-1|0)|0]!=gf[n+Q|0])break}else b=0;if((0|(n=d-b|0))<=(0|A))break c;u=b+O|0,a=(c+E|0)+b|0,A=n;break c}(0|(b=4+(n-s|0)|0))<=(0|A)||(u=s+O|0,a=s+I|0,A=b)}c:{l:{A:{h:{p:if(!(!uf|1!=(0|(s=Sf[131072+(((65535&c)<<1)+f|0)>>1])))){if(!m){if(m=1,!bf)break p;m:{y:if(!(D>>>0<=(n=W)>>>0)){for(;!(b=T^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24)));)if(!((n=n+4|0)>>>0<D>>>0))break y;n=(Rf(b)>>>3|0)+n|0;break m}if(b=T,!(y>>>0<=n>>>0)){for(;;){if(gf[0|n]!=(255&b))break m;if(b=b>>>8|0,(0|y)==(0|(n=n+1|0)))break}n=y}}J=n+N|0,m=2}if(P=c-1|0,!(2!=(0|m)|P>>>0<H>>>0||(m=2,U-c>>>0<3||(0|T)!=(gf[0|(I=P+((Q=P>>>0<U>>>0)?K:E)|0)]|gf[I+1|0]<<8|(gf[I+2|0]<<16|gf[I+3|0]<<24))))){if(tf=wf[f+262160>>2],(c=(s=Q?F:y)-3|0)>>>0<=(n=m=I+4|0)>>>0)break h;for(;!(b=T^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24)));)if(!((n=n+4|0)>>>0<c>>>0))break h;n=(Rf(b)>>>3|0)+n|0;break A}}c=c-s|0;break l}if(b=T,!(s>>>0<=n>>>0)){for(;;){if(gf[0|n]!=(255&b))break A;if(b=b>>>8|0,(0|s)==(0|(n=n+1|0)))break}n=s}}if(w=K+tf|0,m=4+(n-m|0)|0,U>>>0<=P>>>0)b=l;else{if((0|s)==(m+I|0)){b=Cf(T,m<<3);A:{h:if(!(D>>>0<=(n=l)>>>0)){for(;!(s=(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))^b);)if(!((n=n+4|0)>>>0<D>>>0))break h;n=(Rf(s)>>>3|0)+n|0;break A}if(!(y>>>0<=n>>>0)){for(;;){if(gf[0|n]!=(255&b))break A;if(b=b>>>8|0,(0|y)==(0|(n=n+1|0)))break}n=y}}m=(m-l|0)+n|0}b=w}for(wf[S>>2]=T,c=(s=b)+4|0,b=I;c>>>0<=(n=b)>>>0&&(0|T)==(gf[0|(b=n-4|0)]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24)););A:if(!(n>>>0<=s>>>0)&&(c=hf,(0|q)==gf[0|(b=n-1|0)]))for(;;){if((n=b)>>>0<=s>>>0){n=s;break A}if(c=c-1|0,gf[0|(b=n-1|0)]!=gf[0|c])break}if(!(U>>>0<=tf>>>0|Q|(0|l)!=(I-(d=I-n|0)|0))){for(I=Cf(T,0-d<<3),c=w+4|0,s=(wf[S>>2]=I)>>>24|0,b=F;c>>>0<=(n=b)>>>0&&(0|I)==(gf[0|(b=n-4|0)]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24)););A:if(!(n>>>0<=w>>>0)&&(c=Af,(0|s)==gf[0|(b=n-1|0)]))for(;;){if((n=b)>>>0<=w>>>0)break A;if(c=c-1|0,gf[0|(b=n-1|0)]!=gf[0|c])break}d=(d+F|0)-n|0}if((n=(P-(w=H>>>0<(b=P-d|0)>>>0?b:H)|0)+m|0)>>>0<J>>>0|J>>>0<m>>>0){if(c=(b=(-1^w)+U>>>0<3)?U:w,m=2,!(b|ef)){if((b=n>>>0<J>>>0?n:J)>>>0<=A>>>0)s=u,n=a,b=A;else if(65535<((s=O)-(n=w+E|0)|0))break c;if(w>>>0<(a=Sf[131072+(((65535&w)<<1)+f|0)>>1])>>>0){u=s,a=n,A=b;break c}c=w-a|0,u=s,a=n,A=b}}else c=(-1^(b=(P-J|0)+m|0))+U>>>0<3?U:b,m=2}if(j=j-1|0,j&&H>>>0<=c>>>0)continue}break}if((0|A)!=(0|v))break u;I=a,w=u}}if(o>>>0<9+(((((m=p-Z|0)>>>0)/255|0)+x|0)+m|0)>>>0&&k)break n;for(n=_>>>0<g>>>0,b=_-p|0,s=x+1|0,15<=m>>>0?(df[0|x]=240,255<=(u=m-15|0)>>>0&&(If(s,255,(A=((a=m-270|0)>>>0)/255|0)+1|0),s=2+(A+x|0)|0,u=a+Bf(A,-255)|0),df[0|s]=u,s=s+1|0):df[0|x]=m<<4,l=n?b:z,A=s+m|0,n=Z,b=s;u=gf[n+4|0]|gf[n+5|0]<<8|(gf[n+6|0]<<16|gf[n+7|0]<<24),a=gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24),df[0|b]=a,df[b+1|0]=a>>>8,df[b+2|0]=a>>>16,df[b+3|0]=a>>>24,df[b+4|0]=u,df[b+5|0]=u>>>8,df[b+6|0]=u>>>16,df[b+7|0]=u>>>24,n=n+8|0,(b=b+8|0)>>>0<A>>>0;);if(a=p-L|0,df[0|A]=a,df[A+1|0]=a>>>8,o>>>0<6+((c=A+2|0)+(((b=l-4|0)>>>0)/255|0)|0)>>>0&&k)break n;if(a=gf[0|x],15<=b>>>0?(df[0|x]=a+15,510<=(n=l-19|0)>>>0&&(If(c,255,(a=(b=((n=l-529|0)>>>0)/510|0)<<1)+2|0),c=4+((a+m|0)+s|0)|0,n=n+Bf(b,-510)|0),255<=n>>>0&&(df[0|c]=255,c=c+1|0,n=n-255|0),df[0|c]=n,c=c+1|0):df[0|x]=a+b,!(!k|9+(((((n=_-(g=l+p|0)|0)>>>0)/255|0)+c|0)+n|0)>>>0<=o>>>0))break k;for(l=c+1|0,15<=n>>>0?(df[0|c]=240,255<=(b=n-15|0)>>>0&&(If(l,255,(b=((a=n-270|0)>>>0)/255|0)+1|0),l=2+(b+c|0)|0,b=a+Bf(b,-255)|0),df[0|l]=b,l=l+1|0):df[0|c]=n<<4,A=n+l|0,n=g,b=l;u=gf[n+4|0]|gf[n+5|0]<<8|(gf[n+6|0]<<16|gf[n+7|0]<<24),a=gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24),df[0|b]=a,df[b+1|0]=a>>>8,df[b+2|0]=a>>>16,df[b+3|0]=a>>>24,df[b+4|0]=u,df[b+5|0]=u>>>8,df[b+6|0]=u>>>16,df[b+7|0]=u>>>24,n=n+8|0,(b=b+8|0)>>>0<A>>>0;);if(a=_-h|0,df[0|A]=a,df[A+1|0]=a>>>8,!(!k|6+((p=A+2|0)+(((b=v-4|0)>>>0)/255|0)|0)>>>0<=o>>>0))break k;if(a=gf[0|c],15<=b>>>0){df[0|c]=a+15,510<=(n=v-19|0)>>>0&&(If(p,255,(a=(b=((n=v-529|0)>>>0)/510|0)<<1)+2|0),p=4+(((a+_|0)-g|0)+l|0)|0,n=n+Bf(b,-510)|0),255<=n>>>0&&(df[0|p]=255,p=p+1|0,n=n-255|0);break b}df[0|c]=a+b,g=M;break a}if(X>>>0<=u>>>0)break s;if(I=a,!((w=u)>>>0<g>>>0))break}if(g>>>0<=_>>>0||(3<(0|(v=v-(b=g-_|0)|0))?(h=b+h|0,_=g):(_=u,h=a,v=A)),o>>>0<9+(((((w=p-Z|0)>>>0)/255|0)+x|0)+w|0)>>>0&&k)break n;for(l=x+1|0,15<=w>>>0?(df[0|x]=240,255<=(n=w-15|0)>>>0&&(If(l,255,(n=((b=w-270|0)>>>0)/255|0)+1|0),l=2+(n+x|0)|0,n=b+Bf(n,-255)|0),df[0|l]=n,l=l+1|0):df[0|x]=w<<4,m=l+w|0,n=Z,b=l;c=gf[n+4|0]|gf[n+5|0]<<8|(gf[n+6|0]<<16|gf[n+7|0]<<24),s=gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24),df[0|b]=s,df[b+1|0]=s>>>8,df[b+2|0]=s>>>16,df[b+3|0]=s>>>24,df[b+4|0]=c,df[b+5|0]=c>>>8,df[b+6|0]=c>>>16,df[b+7|0]=c>>>24,n=n+8|0,(b=b+8|0)>>>0<m>>>0;);if(b=p-L|0,df[0|m]=b,df[m+1|0]=b>>>8,o>>>0<6+((n=m+2|0)+(((s=z-4|0)>>>0)/255|0)|0)>>>0&&k)break n;b=gf[0|x],x=15<=s>>>0?(df[0|x]=b+15,510<=(b=z-19|0)>>>0&&(If(n,255,(b=(s=((p=z-529|0)>>>0)/510|0)<<1)+2|0),n=4+((b+w|0)+l|0)|0,b=p+Bf(s,-510)|0),255<=b>>>0&&(df[0|n]=255,n=n+1|0,b=b-255|0),df[0|n]=b,n+1|0):(df[0|x]=b+s,n),w=u,I=a,z=_,B=h;continue t}if(g>>>0<=_>>>0?(s=z,b=v):(b=v,17<(0|(s=_-p|0))||(0|(n=(s=((b=v)+_|0)-4>>>0<$>>>0?(b+s|0)-4|0:nf)+(p-_|0)|0))<1||(h=n+h|0,_=n+_|0,b=v-n|0)),z=b,!(o>>>0<9+(((((m=p-Z|0)>>>0)/255|0)+x|0)+m|0)>>>0&&k)){for(l=x+1|0,15<=m>>>0?(df[0|x]=240,255<=(n=m-15|0)>>>0&&(If(n=l,255,(l=((b=m-270|0)>>>0)/255|0)+1|0),n=b+Bf(l,-255)|0,l=2+(l+x|0)|0),df[0|l]=n,l=l+1|0):df[0|x]=m<<4,v=l+m|0,n=Z,b=l;c=gf[n+4|0]|gf[n+5|0]<<8|(gf[n+6|0]<<16|gf[n+7|0]<<24),g=gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24),df[0|b]=g,df[b+1|0]=g>>>8,df[b+2|0]=g>>>16,df[b+3|0]=g>>>24,df[b+4|0]=c,df[b+5|0]=c>>>8,df[b+6|0]=c>>>16,df[b+7|0]=c>>>24,n=n+8|0,(b=b+8|0)>>>0<v>>>0;);if(b=p-L|0,df[0|v]=b,df[v+1|0]=b>>>8,!(o>>>0<6+((n=v+2|0)+(((g=s-4|0)>>>0)/255|0)|0)>>>0&&k)){b=gf[0|x],Z=(x=15<=g>>>0?(df[0|x]=b+15,510<=(b=s-19|0)>>>0&&(If(n,255,(b=(g=((c=s-529|0)>>>0)/510|0)<<1)+2|0),n=4+((b+m|0)+l|0)|0,b=c+Bf(g,-510)|0),255<=b>>>0&&(df[0|n]=255,n=n+1|0,b=b-255|0),df[0|n]=b,n+1|0):(df[0|x]=b+g,n),s+p|0),p=_,L=h,w=u,I=a;continue}}}break}break}}g=Z}c=x}if(p=c,2!=((n=0)|k))break r;break i}df[0|p]=n,p=p+1|0,g=M}if(!(M>>>0<=kf>>>0))break}if(n=sf-g|0,a=(n+240>>>0)/255|0,k&&!((b=1+((a+n|0)+p|0)|0)>>>0<=(a=pf?o+5|0:cf)>>>0)){if(1==((n=0)|k))break r;n=(a=a+(-1^p)|0)-((a+240>>>0)/255|0)|0}k=n+g|0,15<=n>>>0?(df[0|p]=240,a=p+1|0,(b=n-15|0)>>>0<255?df[0|(p=a)]=b:(If(u=a,255,(b=((a=n-270|0)>>>0)/255|0)+1|0),df[0|(p=2+(b+p|0)|0)]=a+Bf(b,-255))):df[0|p]=n<<4,a=Ef(p+1|0,g,n),wf[i>>2]=k-r,n=(a+n|0)-e|0}else{H=gf[f+262170|0],$=e+a|(wf[i>>2]=0),uf=(q=2==(0|k))?$-5|0:$,_=e;i:if(!((ef=(Q=A+(j=r)|0)-12|0)>>>0<j>>>0)){for(nf=H?-1:0,pf=(a=wf[b+1768>>2])>>>0<4095?a:4095,K=(g=Q-5|0)-1|0,w=g-3|0,x=z=W=D=af=T=65596+S|3,ff=f+131072|0,bf=(0|u)<12,C=r;;){if(E=wf[f+262148>>2],l=C-E|0,p=wf[f+262160>>2],h=l>>>0<p+65536>>>0,s=l-65535|0,Z=wf[f+262156>>2],A=Z+E|0,v=gf[0|C]|gf[C+1|0]<<8|(gf[C+2|0]<<16|gf[C+3|0]<<24),P=wf[f+262152>>2],n=wf[f+262164>>2],!(l>>>0<=n>>>0)&&(u=(-1^n)+C|0,(C-n|0)-E&1&&(b=(Bf(gf[0|(a=n+E|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24),-1640531535)>>>15&131068)+f|0,a=n-wf[b>>2]|0,vf[131072+(((65535&n)<<1)+f|0)>>1]=a>>>0<65535?a:65535,n=(wf[b>>2]=n)+1|0),(0|u)!=(0|E)))for(;b=(Bf(gf[0|(a=n+E|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24),-1640531535)>>>15&131068)+f|0,a=n-wf[b>>2]|0,vf[ff+((65535&n)<<1)>>1]=a>>>0<65535?a:65535,wf[b>>2]=n,b=(Bf(gf[0|(a=(u=n+1|0)+E|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24),-1640531535)>>>15&131068)+f|0,a=u-wf[b>>2]|0,vf[ff+((65535&u)<<1)>>1]=a>>>0<65535?a:65535,wf[b>>2]=u,(n=n+2|0)>>>0<l>>>0;);for(N=h?p:s,B=C-j|0,wf[f+262164>>2]=l,hf=(65535&v)==(v>>>16|0)&(0|(Af=v>>>24|0))==(255&v),Y=(d=0)-C|0,I=Z+P|0,L=A+4|0,p=C+8|0,h=C+4|0,G=C-1|0,s=wf[(Bf(gf[0|C]|gf[C+1|0]<<8|(gf[C+2|0]<<16|gf[C+3|0]<<24),-1640531535)>>>15&131068)+f>>2],m=3,c=U=M=0,V=t;;){a:if(!(!V|s>>>0<N>>>0)){b:if(!(l-s>>>(u=0)<8&&H)){k:{o:{t:{n:{if(Z>>>0<=s>>>0){if((0|(b=gf[0|(a=m+G|0)]|gf[a+1|0]<<8))!=(gf[0|(a=((y=s+E|0)+m|0)-1|0)]|gf[a+1|0]<<8)|(0|v)!=(gf[0|y]|gf[y+1|0]<<8|(gf[y+2|0]<<16|gf[y+3|0]<<24)))break b;if(n=y+4|0,(b=w)>>>0<=h>>>0)a=h;else{if(a=(gf[0|h]|gf[h+1|0]<<8|(gf[h+2|0]<<16|gf[h+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24)))break n;n=n+4|0,a=p}if(a>>>0<b>>>0)for(;;){if(b=(gf[0|a]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))){n=((Rf(b)>>>3|0)+a|0)-h|0;break k}if(n=n+4|0,!((a=a+4|0)>>>0<w>>>0))break}(gf[0|n]|gf[n+1|0]<<8)!=(gf[0|a]|gf[a+1|0]<<8)|K>>>0<=a>>>0||(n=n+2|0,a=a+2|0),a>>>0<g>>>0&&(a=gf[0|n]==gf[0|a]?a+1|0:a),n=a-h|0;break k}if((0|v)!=(gf[0|(a=s+P|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24)))break b;if(n=a+4|0,!((y=(b=g>>>0<(F=(Z-s|0)+C|0)>>>0?g:F)-3|0)>>>0<=(u=a=h)>>>0)){if(a=(gf[0|h]|gf[h+1|0]<<8|(gf[h+2|0]<<16|gf[h+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24)))break t;n=n+4|0,u=p}if((a=u)>>>0<y>>>0)for(;;){if(u=(gf[0|a]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))){a=((Rf(u)>>>3|0)+a|0)-h|0;break o}if(n=n+4|0,!((a=a+4|0)>>>0<y>>>0))break}(gf[0|n]|gf[n+1|0]<<8)!=(gf[0|a]|gf[a+1|0]<<8)|b-1>>>0<=a>>>0||(n=n+2|0,a=a+2|0),a>>>0<b>>>0&&(a=gf[0|n]==gf[0|a]?a+1|0:a),a=a-h|0;break o}n=Rf(a)>>>3|0;break k}a=Rf(a)>>>3|0}if(O=s+E|0,!((0|b)!=(C+(u=a+4|0)|0)|g>>>0<=F>>>0)){a=A;o:{t:{if((n=b)>>>0<w>>>0){if(a=(gf[0|b]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24))^(gf[0|A]|gf[A+1|0]<<8|(gf[A+2|0]<<16|gf[A+3|0]<<24)))break t;n=b+4|0,a=L}if(n>>>0<w>>>0)for(;;){if(y=(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))^(gf[0|a]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24))){a=((Rf(y)>>>3|0)+n|0)-b|0;break o}if(a=a+4|0,!((n=n+4|0)>>>0<w>>>0))break}(gf[0|a]|gf[a+1|0]<<8)!=(gf[0|n]|gf[n+1|0]<<8)|K>>>0<=n>>>0||(n=n+2|0,a=a+2|0),n>>>0<g>>>0&&(n=gf[0|a]==gf[0|n]?n+1|0:n),a=n-b|0;break o}a=Rf(a)>>>3|0}u=a+u|0}d=(a=(0|m)<(0|u))?O:d,m=a?u:m;break b}m=(a=(0|m)<(0|(u=n+4|0)))?u:m,d=a?y:d}b:{if(!((0|u)!=(0|m)|(0|u)<4|l>>>0<s+m>>>0)){for(F=u-3|0,n=0,b=16,a=1;a=(O=a>>>0<(y=Sf[131072+(((n+s&65535)<<1)+f|0)>>1])>>>0)?y:a,c=O?n:c,y=b>>4,b=O?16:b+1|0,(0|(n=n+y|0))<(0|F););if(n=(b=s>>>0<a>>>0)?0:a,s=s-((a=1<a>>>0)?n:0)|0,a){n=b?3:2,m=u;break b}}k:{o:{t:if(!(1!=Sf[131072+(((65535&s)<<1)+f|0)>>1]|c)){if(!U){if(U=1,!hf)break t;n:{s:if(!(w>>>0<=(n=h)>>>0)){for(;!(a=v^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24)));)if(!((n=n+4|0)>>>0<w>>>0))break s;n=(Rf(a)>>>3|0)+n|0;break n}if(a=v,!(g>>>0<=n>>>0)){for(;;){if(gf[0|n]!=(255&a))break n;if(a=a>>>8|0,(0|g)==(0|(n=n+1|0)))break}n=g}}U=2,M=n+Y|0}if(R=s-1|0,!(2!=(0|U)|R>>>0<N>>>0||(U=2,Z-s>>>0<3||(0|v)!=(gf[0|(y=R+((O=R>>>0<Z>>>0)?P:E)|0)]|gf[y+1|0]<<8|(gf[y+2|0]<<16|gf[y+3|0]<<24))))){if(F=wf[f+262160>>2],(u=(b=O?I:g)-3|0)>>>0<=(n=s=y+4|0)>>>0)break o;for(;!(a=v^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24)));)if(!((n=n+4|0)>>>0<u>>>0))break o;n=(Rf(a)>>>3|0)+n|0;break k}}s=s-Sf[131072+(((s+c&65535)<<1)+f|0)>>1]|0,n=0;break b}if(a=v,!(b>>>0<=n>>>0)){for(;;){if(gf[0|n]!=(255&a))break k;if(a=a>>>8|0,(0|b)==(0|(n=n+1|0)))break}n=b}}if(c=F+P|0,s=4+(n-s|0)|0,Z>>>0<=R>>>0)a=A;else{if((0|b)==(s+y|0)){a=Cf(v,s<<3);k:{o:if(!(w>>>0<=(n=A)>>>0)){for(;!(b=(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))^a);)if(!((n=n+4|0)>>>0<w>>>0))break o;n=(Rf(b)>>>3|0)+n|0;break k}if(!(g>>>0<=n>>>0)){for(;;){if(gf[0|n]!=(255&a))break k;if(a=a>>>8|0,(0|g)==(0|(n=n+1|0)))break}n=g}}s=(s-A|0)+n|0}a=c}for(wf[65596+S>>2]=v,b=(u=a)+4|0,a=y;b>>>0<=(n=a)>>>0&&(0|v)==(gf[0|(a=n-4|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24)););k:if(!(n>>>0<=u>>>0)&&(b=x,(0|Af)==gf[0|(a=n-1|0)]))for(;;){if((n=a)>>>0<=u>>>0){n=u;break k}if(b=b-1|0,gf[0|(a=n-1|0)]!=gf[0|b])break}if(!(Z>>>0<=F>>>0|O|(0|A)!=(y-(U=y-n|0)|0))){for(y=Cf(v,0-U<<3),b=c+4|0,u=(wf[65596+S>>2]=y)>>>24|0,a=I;b>>>0<=(n=a)>>>0&&(0|y)==(gf[0|(a=n-4|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24)););k:if(!(n>>>0<=c>>>0)&&(b=z,(0|u)==gf[0|(a=n-1|0)]))for(;;){if((n=a)>>>0<=c>>>0)break k;if(b=b-1|0,gf[0|(a=n-1|0)]!=gf[0|b])break}U=(I+U|0)-n|0}if((a=(R-(y=N>>>0<(a=R-U|0)>>>0?a:N)|0)+s|0)>>>0<M>>>0|M>>>0<s>>>0)if(n=2,Z+(-1^y)>>>(c=0)<3)U=2,s=Z;else{if((b=a>>>0<M>>>0?a:M)>>>0<=m>>>0)u=d,b=m;else if(65535<(C-(u=y+E|0)|0))break a;if(y>>>0<(a=Sf[131072+(((65535&y)<<1)+f|0)>>1])>>>0){d=u,m=b;break a}s=y-a|0,d=u,U=2,m=b}else s=Z+(-1^(a=(R-M|0)+s|0))>>>0<3?Z:a,c=0,U=n=2}if(V=V-1|0,3!=(0|n))continue}break}a:{b:{k:{o:{t:{if(4<=(0|m)){if(s=C-d|0,pf>>>0<(V=H&&m-19>>>0<18?18:m)>>>0)break t;if(u=14<(0|B))break o;a=B+1|0,b=B;break k}C=C+1|0;break b}if(uf>>>0<9+(B+(((B>>>0)/255|0)+_|0)|0)>>>0&&k)break a;for(c=_+1|0,15<=B>>>0?(df[0|_]=240,255<=(n=B-15|0)>>>0&&(If(c,255,(b=((a=(C-j|0)-270|0)>>>0)/255|0)+1|0),c=2+(b+_|0)|0,n=a+Bf(b,-255)|0),df[0|c]=n,c=c+1|0):df[0|_]=B<<4,A=c+B|0,n=j,a=c;u=gf[n+4|0]|gf[n+5|0]<<8|(gf[n+6|0]<<16|gf[n+7|0]<<24),b=gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24),df[0|a]=b,df[a+1|0]=b>>>8,df[a+2|0]=b>>>16,df[a+3|0]=b>>>24,df[a+4|0]=u,df[a+5|0]=u>>>8,df[a+6|0]=u>>>16,df[a+7|0]=u>>>24,n=n+8|0,(a=a+8|0)>>>0<A>>>0;);if(df[0|A]=s,df[A+1|0]=s>>>8,uf>>>0<6+((n=A+2|0)+(((b=V-4|0)>>>0)/255|0)|0)>>>0&&k)break a;if(a=gf[0|_],15<=b>>>0){df[0|_]=a+15,510<=(a=V-19|0)>>>0&&(If(u=n,255,(b=(a=((n=V-529|0)>>>0)/510|0)<<1)+2|0),a=n+Bf(a,-510)|0,n=4+((b+B|0)+c|0)|0),255<=a>>>0&&(df[0|n]=255,n=n+1|0,a=a-255|0),df[0|n]=a,_=n+1|0,j=C=C+V|0;break b}df[0|_]=a+b,j=C=C+V|0,_=n;break b}b=(a=B+1|0)+((B-15|0)/255|0)|0}wf[12+S>>2]=B,wf[4+S>>2]=0,wf[8+S>>2]=1,wf[S>>2]=b,wf[28+S>>2]=a,wf[20+S>>2]=0,wf[24+S>>2]=1,14<(0|(b=a))&&(b=1+(((a-15|0)/255|0)+a|0)|0),wf[(n=S)+16>>2]=b,b=B+2|0;k:{if(13<=(0|B))wf[44+S>>2]=b,wf[36+S>>2]=0,wf[40+S>>2]=1,c=B+3|0,wf[32+S>>2]=c+((B-13|0)/255|0);else if(wf[44+S>>2]=b,wf[36+S>>2]=0,wf[40+S>>2]=1,wf[32+S>>2]=b,c=15,12!=(0|B)){b=c=B+3|0;break k}b=4+(B+((B-12|0)/255|0)|0)|0}if(wf[60+S>>2]=c,wf[52+S>>2]=0,wf[56+S>>2]=1,wf[48+S>>2]=b,u)for(A=(a=((B-15|0)/255|0)+a|0)+(n=4)|0,u=a+3|0;a=u,wf[(b=(n<<4)+S|0)+12>>2]=B,wf[b+4>>2]=s,19<=(wf[b+8>>2]=n)>>>0&&(a=A+((n-19|0)/255|0)|0),wf[b>>2]=a,a=(0|n)==(0|V),n=n+1|0,!a;);else for(u=c+1|0,n=4;a=c,wf[(b=(n<<4)+S|0)+12>>2]=B,wf[b+4>>2]=s,19<=(wf[b+8>>2]=n)>>>0&&(a=u+((n-19|0)/255|0)|0),wf[b>>2]=a,a=(0|n)==(0|V),n=n+1|0,!a;);wf[(b=(V<<4)+S|0)+28>>2]=1,wf[b+20>>2]=0,wf[b+24>>2]=1,wf[b+36>>2]=0,wf[b+40>>2]=1,wf[b+44>>2]=2,wf[b+60>>2]=3,wf[b+52>>2]=0,wf[b+56>>2]=1,a=wf[b>>2],wf[b+16>>2]=a+1,wf[b+32>>2]=a+2,wf[b+48>>2]=a+3;k:{o:if(!((0|V)<2|ef>>>0<(J=C+1|0)>>>0))for(tf=-1^E,Y=1;;){sf=wf[(of=(a=Y<<4)+S|0)>>2],cf=wf[((M=Y+1|0)<<4)+S>>2];t:{n:{s:{u:{if(!bf){if((0|sf)<(0|cf)|wf[(a+S|0)+64>>2]>=(sf+3|0))break u;m=l;break t}if((0|sf)<(0|cf))break s;m=l;break t}if(m=J-E|0,h=wf[f+262160>>2],s=m>>>0<h+65536>>>0,u=m-65535|0,F=wf[f+262156>>2],A=F+E|0,I=gf[0|J]|gf[J+1|0]<<8|(gf[J+2|0]<<16|gf[J+3|0]<<24),!(m>>>0<=l>>>0)&&((J-E|0)-(n=l)&1&&(b=(Bf(gf[0|(a=l+E|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24),-1640531535)>>>15&131068)+f|0,a=l-wf[b>>2]|0,vf[131072+(((65535&l)<<1)+f|0)>>1]=a>>>0<65535?a:65535,n=(wf[b>>2]=l)+1|0),(J+tf|0)!=(0|l)))for(;b=(Bf(gf[0|(a=n+E|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24),-1640531535)>>>15&131068)+f|0,a=n-wf[b>>2]|0,vf[ff+((65535&n)<<1)>>1]=a>>>0<65535?a:65535,wf[b>>2]=n,b=(Bf(gf[0|(a=(l=n+1|0)+E|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24),-1640531535)>>>15&131068)+f|0,a=l-wf[b>>2]|0,vf[ff+((65535&l)<<1)>>1]=a>>>0<65535?a:65535,wf[b>>2]=l,(n=n+2|0)>>>0<m>>>0;);for(X=s?h:u,wf[f+262164>>2]=m,O=(65535&I)==(I>>>16|0)&(0|(N=I>>>24|0))==(255&I),Af=(p=0)-J|0,Z=F+P|0,y=A+4|0,L=J+8|0,v=J+4|0,hf=J-1|0,s=wf[(Bf(gf[0|J]|gf[J+1|0]<<8|(gf[J+2|0]<<16|gf[J+3|0]<<24),-1640531535)>>>15&131068)+f>>2],h=3,c=d=U=0,l=t;;){u:if(!(!l|s>>>0<X>>>0)){c:if(!(m-s>>>(u=0)<8&&H)){l:{A:{h:{p:{if(F>>>0<=s>>>0){if((0|(b=gf[0|(a=h+hf|0)]|gf[a+1|0]<<8))!=(gf[0|(a=((B=s+E|0)+h|0)-1|0)]|gf[a+1|0]<<8)|(0|I)!=(gf[0|B]|gf[B+1|0]<<8|(gf[B+2|0]<<16|gf[B+3|0]<<24)))break c;if(n=B+4|0,(b=w)>>>0<=v>>>0)a=v;else{if(a=(gf[0|v]|gf[v+1|0]<<8|(gf[v+2|0]<<16|gf[v+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24)))break p;n=n+4|0,a=L}if(a>>>0<b>>>0)for(;;){if(b=(gf[0|a]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))){n=((Rf(b)>>>3|0)+a|0)-v|0;break l}if(n=n+4|0,!((a=a+4|0)>>>0<w>>>0))break}(gf[0|n]|gf[n+1|0]<<8)!=(gf[0|a]|gf[a+1|0]<<8)|K>>>0<=a>>>0||(n=n+2|0,a=a+2|0),a>>>0<g>>>0&&(a=gf[0|n]==gf[0|a]?a+1|0:a),n=a-v|0;break l}if((0|I)!=(gf[0|(a=s+P|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24)))break c;if(n=a+4|0,!((B=(b=g>>>0<(G=(F-s|0)+J|0)>>>0?g:G)-3|0)>>>0<=(u=a=v)>>>0)){if(a=(gf[0|v]|gf[v+1|0]<<8|(gf[v+2|0]<<16|gf[v+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24)))break h;n=n+4|0,u=L}if((a=u)>>>0<B>>>0)for(;;){if(u=(gf[0|a]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))){a=((Rf(u)>>>3|0)+a|0)-v|0;break A}if(n=n+4|0,!((a=a+4|0)>>>0<B>>>0))break}(gf[0|n]|gf[n+1|0]<<8)!=(gf[0|a]|gf[a+1|0]<<8)|b-1>>>0<=a>>>0||(n=n+2|0,a=a+2|0),a>>>0<b>>>0&&(a=gf[0|n]==gf[0|a]?a+1|0:a),a=a-v|0;break A}n=Rf(a)>>>3|0;break l}a=Rf(a)>>>3|0}if(R=s+E|0,!((0|b)!=(J+(u=a+4|0)|0)|g>>>0<=G>>>0)){a=A;A:{h:{if((n=b)>>>0<w>>>0){if(a=(gf[0|b]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24))^(gf[0|A]|gf[A+1|0]<<8|(gf[A+2|0]<<16|gf[A+3|0]<<24)))break h;n=b+4|0,a=y}if(n>>>0<w>>>0)for(;;){if(B=(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))^(gf[0|a]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24))){a=((Rf(B)>>>3|0)+n|0)-b|0;break A}if(a=a+4|0,!((n=n+4|0)>>>0<w>>>0))break}(gf[0|a]|gf[a+1|0]<<8)!=(gf[0|n]|gf[n+1|0]<<8)|K>>>0<=n>>>0||(n=n+2|0,a=a+2|0),n>>>0<g>>>0&&(n=gf[0|a]==gf[0|n]?n+1|0:n),a=n-b|0;break A}a=Rf(a)>>>3|0}u=a+u|0}p=(a=(0|h)<(0|u))?R:p,h=a?u:h;break c}h=(a=(0|h)<(0|(u=n+4|0)))?u:h,p=a?B:p}c:{if(!((0|u)!=(0|h)|(0|u)<4|m>>>0<s+h>>>0)){for(G=u-3|0,n=0,b=16,a=1;a=(R=a>>>0<(B=Sf[131072+(((n+s&65535)<<1)+f|0)>>1])>>>0)?B:a,c=R?n:c,B=b>>4,b=R?16:b+1|0,(0|(n=n+B|0))<(0|G););if(n=(b=s>>>0<a>>>0)?0:a,s=s-((a=1<a>>>0)?n:0)|0,a){n=b?3:2,h=u;break c}}l:{A:{h:if(!(1!=Sf[131072+(((65535&s)<<1)+f|0)>>1]|c)){if(!d){if(d=1,!O)break h;p:{m:if(!(w>>>0<=(n=v)>>>0)){for(;!(a=I^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24)));)if(!((n=n+4|0)>>>0<w>>>0))break m;n=(Rf(a)>>>3|0)+n|0;break p}if(a=I,!(g>>>0<=n>>>0)){for(;;){if(gf[0|n]!=(255&a))break p;if(a=a>>>8|0,(0|g)==(0|(n=n+1|0)))break}n=g}}U=n+Af|0,d=2}if(rf=s-1|0,!(2!=(0|d)|rf>>>0<X>>>0||(d=2,F-s>>>0<3||(0|I)!=(gf[0|(B=rf+((R=rf>>>0<F>>>0)?P:E)|0)]|gf[B+1|0]<<8|(gf[B+2|0]<<16|gf[B+3|0]<<24))))){if(G=wf[f+262160>>2],(u=(b=R?Z:g)-3|0)>>>0<=(n=s=B+4|0)>>>0)break A;for(;!(a=I^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24)));)if(!((n=n+4|0)>>>0<u>>>0))break A;n=(Rf(a)>>>3|0)+n|0;break l}}s=s-Sf[131072+(((s+c&65535)<<1)+f|0)>>1]|0,n=0;break c}if(a=I,!(b>>>0<=n>>>0)){for(;;){if(gf[0|n]!=(255&a))break l;if(a=a>>>8|0,(0|b)==(0|(n=n+1|0)))break}n=b}}if(c=P+G|0,s=4+(n-s|0)|0,F>>>0<=rf>>>0)a=A;else{if((0|b)==(s+B|0)){a=Cf(I,s<<3);l:{A:if(!(w>>>0<=(n=A)>>>0)){for(;!(b=(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))^a);)if(!((n=n+4|0)>>>0<w>>>0))break A;n=(Rf(b)>>>3|0)+n|0;break l}if(!(g>>>0<=n>>>0)){for(;;){if(gf[0|n]!=(255&a))break l;if(a=a>>>8|0,(0|g)==(0|(n=n+1|0)))break}n=g}}s=(s-A|0)+n|0}a=c}for(wf[65596+S>>2]=I,b=(u=a)+4|0,a=B;b>>>0<=(n=a)>>>0&&(0|I)==(gf[0|(a=n-4|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24)););l:if(!(n>>>0<=u>>>0)&&(b=W,(0|N)==gf[0|(a=n-1|0)]))for(;;){if((n=a)>>>0<=u>>>0){n=u;break l}if(b=b-1|0,gf[0|(a=n-1|0)]!=gf[0|b])break}if(!(F>>>0<=G>>>0|R|(0|A)!=(B-(d=B-n|0)|0))){for(B=Cf(I,0-d<<3),b=c+4|0,u=(wf[65596+S>>2]=B)>>>24|0,a=Z;b>>>0<=(n=a)>>>0&&(0|B)==(gf[0|(a=n-4|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24)););l:if(!(n>>>0<=c>>>0)&&(b=D,(0|u)==gf[0|(a=n-1|0)]))for(;;){if((n=a)>>>0<=c>>>0)break l;if(b=b-1|0,gf[0|(a=n-1|0)]!=gf[0|b])break}d=(d+Z|0)-n|0}if((a=(rf-(d=X>>>0<(a=rf-d|0)>>>0?a:X)|0)+s|0)>>>0<U>>>0|U>>>0<s>>>0)if(n=2,F+(-1^d)>>>(c=0)<3)d=2,s=F;else{if((b=a>>>0<U>>>0?a:U)>>>0<=h>>>0)u=p,b=h;else if(65535<(J-(u=d+E|0)|0))break u;if(d>>>0<(a=Sf[131072+(((65535&d)<<1)+f|0)>>1])>>>0){p=u,h=b;break u}s=d-a|0,p=u,d=2,h=b}else s=F+(-1^(a=(rf-U|0)+s|0))>>>0<3?F:a,c=0,d=n=2}if(l=l-1|0,3!=(0|n))continue}break}if((0|h)<4)break t;A=H&&h-19>>>0<18?18:h,b=J-p|0;break n}if(m=J-E|0,s=wf[f+262160>>2],A=m>>>0<s+65536>>>0,u=m-65535|0,G=wf[f+262156>>2],v=G+E|0,y=gf[0|J]|gf[J+1|0]<<8|(gf[J+2|0]<<16|gf[J+3|0]<<24),!(m>>>0<=l>>>0)&&((J-E|0)-(n=l)&1&&(b=(Bf(gf[0|(a=l+E|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24),-1640531535)>>>15&131068)+f|0,a=l-wf[b>>2]|0,vf[131072+(((65535&l)<<1)+f|0)>>1]=a>>>0<65535?a:65535,n=(wf[b>>2]=l)+1|0),(J+tf|0)!=(0|l)))for(;b=(Bf(gf[0|(a=n+E|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24),-1640531535)>>>15&131068)+f|0,a=n-wf[b>>2]|0,vf[ff+((65535&n)<<1)>>1]=a>>>0<65535?a:65535,wf[b>>2]=n,b=(Bf(gf[0|(a=(l=n+1|0)+E|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24),-1640531535)>>>15&131068)+f|0,a=l-wf[b>>2]|0,vf[ff+((65535&l)<<1)>>1]=a>>>0<65535?a:65535,wf[b>>2]=l,(n=n+2|0)>>>0<m>>>0;);for(rf=A?s:u,wf[f+262164>>2]=m,N=(65535&y)==(y>>>16|0)&(0|(R=y>>>24|0))==(255&y),O=(p=0)-J|0,B=P+G|0,Z=v+4|0,I=J+8|0,L=J+4|0,Af=J-1|0,s=wf[(Bf(gf[0|J]|gf[J+1|0]<<8|(gf[J+2|0]<<16|gf[J+3|0]<<24),-1640531535)>>>15&131068)+f>>2],h=t,A=hf=V-Y|(c=d=U=0);;){s:if(!(!h|s>>>0<rf>>>0)){u:if(!(m-s>>>(l=0)<8&&H)){c:{l:{A:{h:{if(G>>>0<=s>>>0){if((0|(b=gf[0|(a=A+Af|0)]|gf[a+1|0]<<8))!=(gf[0|(a=((u=s+E|0)+A|0)-1|0)]|gf[a+1|0]<<8)|(0|y)!=(gf[0|u]|gf[u+1|0]<<8|(gf[u+2|0]<<16|gf[u+3|0]<<24)))break u;if(n=u+4|0,(b=w)>>>0<=L>>>0)a=L;else{if(a=(gf[0|L]|gf[L+1|0]<<8|(gf[L+2|0]<<16|gf[L+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24)))break h;n=n+4|0,a=I}if(a>>>0<b>>>0)for(;;){if(b=(gf[0|a]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))){n=((Rf(b)>>>3|0)+a|0)-L|0;break c}if(n=n+4|0,!((a=a+4|0)>>>0<w>>>0))break}(gf[0|n]|gf[n+1|0]<<8)!=(gf[0|a]|gf[a+1|0]<<8)|K>>>0<=a>>>0||(n=n+2|0,a=a+2|0),a>>>0<g>>>0&&(a=gf[0|n]==gf[0|a]?a+1|0:a),n=a-L|0;break c}if((0|y)!=(gf[0|(a=s+P|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24)))break u;if(n=a+4|0,!((l=(b=g>>>0<(F=(G-s|0)+J|0)>>>0?g:F)-3|0)>>>0<=(u=a=L)>>>0)){if(a=(gf[0|L]|gf[L+1|0]<<8|(gf[L+2|0]<<16|gf[L+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24)))break A;n=n+4|0,u=I}if((a=u)>>>0<l>>>0)for(;;){if(u=(gf[0|a]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))){a=((Rf(u)>>>3|0)+a|0)-L|0;break l}if(n=n+4|0,!((a=a+4|0)>>>0<l>>>0))break}(gf[0|n]|gf[n+1|0]<<8)!=(gf[0|a]|gf[a+1|0]<<8)|b-1>>>0<=a>>>0||(n=n+2|0,a=a+2|0),a>>>0<b>>>0&&(a=gf[0|n]==gf[0|a]?a+1|0:a),a=a-L|0;break l}n=Rf(a)>>>3|0;break c}a=Rf(a)>>>3|0}if(lf=s+E|0,!((0|b)!=(J+(l=a+4|0)|0)|g>>>0<=F>>>0)){a=v;l:{A:{if((n=b)>>>0<w>>>0){if(a=(gf[0|b]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24))^(gf[0|v]|gf[v+1|0]<<8|(gf[v+2|0]<<16|gf[v+3|0]<<24)))break A;n=b+4|0,a=Z}if(n>>>0<w>>>0)for(;;){if(u=(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))^(gf[0|a]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24))){a=((Rf(u)>>>3|0)+n|0)-b|0;break l}if(a=a+4|0,!((n=n+4|0)>>>0<w>>>0))break}(gf[0|a]|gf[a+1|0]<<8)!=(gf[0|n]|gf[n+1|0]<<8)|K>>>0<=n>>>0||(n=n+2|0,a=a+2|0),n>>>0<g>>>0&&(n=gf[0|a]==gf[0|n]?n+1|0:n),a=n-b|0;break l}a=Rf(a)>>>3|0}l=a+l|0}p=(a=(0|A)<(0|l))?lf:p,A=a?l:A;break u}A=(a=(0|A)<(0|(l=n+4|0)))?l:A,p=a?u:p}u:{if(!((0|l)!=(0|A)|(0|l)<4|m>>>0<s+A>>>0)){for(F=l-3|0,n=0,b=16,a=1;a=(X=a>>>0<(u=Sf[131072+(((n+s&65535)<<1)+f|0)>>1])>>>0)?u:a,c=X?n:c,u=b>>4,b=X?16:b+1|0,(0|(n=n+u|0))<(0|F););if(n=(b=s>>>0<a>>>0)?0:a,s=s-((a=1<a>>>0)?n:0)|0,a){n=b?3:2,A=l;break u}}c:{l:{A:if(!(1!=Sf[131072+(((65535&s)<<1)+f|0)>>1]|c)){if(!d){if(d=1,!N)break A;h:{p:if(!(w>>>0<=(n=L)>>>0)){for(;!(a=y^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24)));)if(!((n=n+4|0)>>>0<w>>>0))break p;n=(Rf(a)>>>3|0)+n|0;break h}if(a=y,!(g>>>0<=n>>>0)){for(;;){if(gf[0|n]!=(255&a))break h;if(a=a>>>8|0,(0|g)==(0|(n=n+1|0)))break}n=g}}U=n+O|0,d=2}if(kf=s-1|0,!(2!=(0|d)|kf>>>0<rf>>>0||(d=2,G-s>>>0<3||(0|y)!=(gf[0|(F=kf+((X=kf>>>0<G>>>0)?P:E)|0)]|gf[F+1|0]<<8|(gf[F+2|0]<<16|gf[F+3|0]<<24))))){if(c=wf[f+262160>>2],(u=(b=X?B:g)-3|0)>>>0<=(n=s=F+4|0)>>>0)break l;for(;!(a=y^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24)));)if(!((n=n+4|0)>>>0<u>>>0))break l;n=(Rf(a)>>>3|0)+n|0;break c}}s=s-Sf[131072+(((s+c&65535)<<1)+f|0)>>1]|0,n=0;break u}if(a=y,!(b>>>0<=n>>>0)){for(;;){if(gf[0|n]!=(255&a))break c;if(a=a>>>8|0,(0|b)==(0|(n=n+1|0)))break}n=b}}if(l=c+P|0,s=4+(n-s|0)|0,G>>>0<=kf>>>0)a=v;else{if((0|b)==(s+F|0)){a=Cf(y,s<<3);c:{l:if(!(w>>>0<=(n=v)>>>0)){for(;!(b=(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))^a);)if(!((n=n+4|0)>>>0<w>>>0))break l;n=(Rf(b)>>>3|0)+n|0;break c}if(!(g>>>0<=n>>>0)){for(;;){if(gf[0|n]!=(255&a))break c;if(a=a>>>8|0,(0|g)==(0|(n=n+1|0)))break}n=g}}s=(s-v|0)+n|0}a=l}for(wf[65596+S>>2]=y,b=(u=a)+4|0,a=F;b>>>0<=(n=a)>>>0&&(0|y)==(gf[0|(a=n-4|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24)););c:if(!(n>>>0<=u>>>0)&&(b=af,(0|R)==gf[0|(a=n-1|0)]))for(;;){if((n=a)>>>0<=u>>>0){n=u;break c}if(b=b-1|0,gf[0|(a=n-1|0)]!=gf[0|b])break}if(!(G>>>0<=c>>>0|X|(0|v)!=(F-(d=F-n|0)|0))){for(c=Cf(y,0-d<<3),b=l+4|0,u=(wf[65596+S>>2]=c)>>>24|0,a=B;b>>>0<=(n=a)>>>0&&(0|c)==(gf[0|(a=n-4|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24)););c:if(!(n>>>0<=l>>>0)&&(b=T,(0|u)==gf[0|(a=n-1|0)]))for(;;){if((n=a)>>>0<=l>>>0)break c;if(b=b-1|0,gf[0|(a=n-1|0)]!=gf[0|b])break}d=(d+B|0)-n|0}if((a=(kf-(l=rf>>>0<(a=kf-d|0)>>>0?a:rf)|0)+s|0)>>>0<U>>>0|U>>>0<s>>>0)if(n=2,G+(-1^l)>>>(c=0)<3)d=2,s=G;else{if((b=a>>>0<U>>>0?a:U)>>>0<=A>>>0)u=p,b=A;else if(65535<(J-(u=l+E|0)|0))break s;if(l>>>0<(a=Sf[131072+(((65535&l)<<1)+f|0)>>1])>>>0){p=u,A=b;break s}s=l-a|0,p=u,d=2,A=b}else s=G+(-1^(a=(kf-U|0)+s|0))>>>0<3?G:a,c=0,d=n=2}if(h=h-1|0,3!=(0|n))continue}break}if((0|A)<=(0|hf))break t;if(b=J-p|0,!H|18<=A-19>>>0){if(!A)break t}else A=18}n:{s:{if(!(4095<(A+Y|0)|pf>>>0<A>>>0)){if(h=(c=wf[of+12>>2])-15|0,!(s=14<(0|c))){if(n=c+1|0,(0|(a=(u=sf-c|0)+(14==(0|c)?16:n)|0))<(0|cf)&&(wf[(l=(Y+1<<4)+S|0)+12>>2]=n,wf[l+4>>2]=0,wf[l+8>>2]=1,wf[l>>2]=a),a=n=c+2|0,12<(0|c)&&(a=3+(c+(((c<<16)-851968>>16)/255<<16>>16)|0)|0),(0|(a=a+u|0))<wf[(l=(Y+2<<4)+S|0)>>2]&&(wf[l+12>>2]=n,wf[l+4>>2]=0,wf[l+8>>2]=1,wf[l>>2]=a),a=n=c+3|0,12<=(0|c)&&(a=4+(c+(((c<<16)-786432>>16)/255<<16>>16)|0)|0),(0|(a=a+u|0))>=wf[(l=(Y+3<<4)+S|0)>>2])break n;wf[l+12>>2]=n,wf[l+4>>2]=0,wf[l+8>>2]=1,wf[l>>2]=a;break n}if((0|(a=(l=sf+((-1^c)-((h>>>0)/255|0)|0)|0)+((u=c+2|0)+((c-14|0)/255|0)|0)|0))<(0|cf)&&(wf[(n=(Y+1<<4)+S|0)+12>>2]=c+1,wf[n+4>>2]=0,wf[n+8>>2]=1,wf[n>>2]=a),(0|(a=l+((n=c+3|0)+((c-13|0)/255|0)|0)|0))<wf[(p=(Y+2<<4)+S|0)>>2]&&(wf[p+12>>2]=u,wf[p+4>>2]=0,wf[p+8>>2]=1,wf[p>>2]=a),(0|(a=4+(l+(c+((c-12|0)/255|0)|0)|0)|0))<wf[(u=(Y+3<<4)+S|0)>>2])break s;break n}V=Y+1|0;break k}wf[u+12>>2]=n,wf[u+4>>2]=0,wf[u+8>>2]=1,wf[u>>2]=a}if(!((0|A)<4))if(a=(h>>>0)/255|0,1!=wf[8+((Y<<(n=4))+S|0)>>2])for(;(0|(a=sf+(19<=n>>>0?4+((n-19|0)/255|0)|0:3)|0))>(nf+wf[((s=n+Y|0)<<4)+S>>2]|0)&&(0|s)<=(V+3|0)||(wf[(u=(s<<4)+S|0)+12>>2]=0,wf[u+4>>2]=b,wf[u+8>>2]=n,wf[u>>2]=a,V=(0|n)==(0|A)&&(0|V)<(0|s)?s:V),a=(0|n)==(0|A),n=n+1|0,!a;);else if(h=(a=s?1+(a+c|0)|0:c)+4|0,u=a+3|0,(0|c)<(0|Y))for(l=(Y-c<<4)+S|0;(0|(a=(a=19<=n>>>0?h+((n-19|0)/255|0)|0:u)+(s=wf[l>>2])|0))>(nf+wf[((p=n+Y|0)<<4)+S>>2]|0)&&(0|p)<=(V+3|0)||(wf[(s=(p<<4)+S|0)+12>>2]=c,wf[s+4>>2]=b,wf[s+8>>2]=n,wf[s>>2]=a,V=(0|n)==(0|A)&&(0|V)<(0|p)?p:V),a=(0|n)==(0|A),n=n+1|0,!a;);else for(;(0|(l=19<=n>>>0?h+((n-19|0)/255|0)|0:u))>(nf+wf[((s=n+Y|0)<<4)+S>>2]|0)&&(0|s)<=(V+3|0)||(wf[(a=(s<<4)+S|0)+12>>2]=c,wf[a+4>>2]=b,wf[a+8>>2]=n,wf[a>>2]=l,V=(0|n)==(0|A)&&(0|V)<(0|s)?s:V),a=(0|n)==(0|A),n=n+1|0,!a;);wf[(b=(V<<4)+S|0)+28>>2]=1,wf[b+20>>2]=0,wf[b+24>>2]=1,wf[b+36>>2]=0,wf[b+40>>2]=1,wf[b+44>>2]=2,wf[b+60>>2]=3,wf[b+52>>2]=0,wf[b+56>>2]=1,a=wf[b>>2],wf[b+16>>2]=a+1,wf[b+32>>2]=a+2,wf[b+48>>2]=a+3}if(ef>>>0<(J=C+M|0)>>>0)break o;if(l=m,!((0|(Y=M))<(0|V)))break}Y=V-(A=wf[(a=(V<<4)+S|0)+8>>2])|0,b=wf[a+4>>2]}for(;n=wf[(u=(Y<<4)+S|0)+8>>2],wf[u+8>>2]=A,a=wf[u+4>>2],wf[u+4>>2]=b,u=(0|n)<=(0|Y),Y=Y-n|0,A=n,b=a,u;);if(!((0|V)<1)){if(u=0,!k)for(;;){if(1==(0|(h=wf[(a=(u<<4)+S|0)+8>>2])))u=u+1|0,C=C+1|0;else{for(c=_+1|0,A=wf[a+4>>2],(l=C-j|0)>>>0<=14?df[0|_]=l<<4:(df[0|_]=240,255<=(n=l-15|0)>>>0&&(If(c,255,(b=((a=l-270|0)>>>0)/255|0)+1|0),c=2+(b+_|0)|0,n=a+Bf(b,-255)|0),df[0|c]=n,c=c+1|0),u=u+h|0,s=c+l|0,n=c;b=gf[j+4|0]|gf[j+5|0]<<8|(gf[j+6|0]<<16|gf[j+7|0]<<24),a=gf[0|j]|gf[j+1|0]<<8|(gf[j+2|0]<<16|gf[j+3|0]<<24),df[0|n]=a,df[n+1|0]=a>>>8,df[n+2|0]=a>>>16,df[n+3|0]=a>>>24,df[n+4|0]=b,df[n+5|0]=b>>>8,df[n+6|0]=b>>>16,df[n+7|0]=b>>>24,j=j+8|0,(n=n+8|0)>>>0<s>>>0;);(df[0|s]=A,df[s+1|0]=A>>>8,n=s+2|0,b=gf[0|_],(a=h-4|0)>>>0<=14)?(df[0|_]=a+b,j=C=h+C|0,_=n):(df[0|_]=b+15,510<=(a=h-19|0)>>>0&&(If(s=n,255,(b=(a=((n=h-529|0)>>>0)/510|0)<<1)+2|0),a=n+Bf(a,-510)|0,n=4+((b+l|0)+c|0)|0),255<=a>>>0&&(df[0|n]=255,n=n+1|0,a=a-255|0),df[0|n]=a,_=n+1|0,j=C=h+C|0)}if(!((0|u)<(0|V)))break b}for(;;){if(1!=(0|(p=wf[(a=(u<<4)+S|0)+8>>2]))){if(uf>>>0<9+(((((c=C-j|0)>>>0)/255|0)+_|0)+c|0)>>>0)break a;for(s=_+1|0,A=wf[a+4>>2],15<=c>>>0?(df[0|_]=240,255<=(n=c-15|0)>>>0&&(If(s,255,(b=((a=c-270|0)>>>0)/255|0)+1|0),s=2+(b+_|0)|0,n=a+Bf(b,-255)|0),df[0|s]=n,s=s+1|0):df[0|_]=c<<4,u=u+p|0,h=s+c|0,n=j,a=s;l=gf[n+4|0]|gf[n+5|0]<<8|(gf[n+6|0]<<16|gf[n+7|0]<<24),b=gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24),df[0|a]=b,df[a+1|0]=b>>>8,df[a+2|0]=b>>>16,df[a+3|0]=b>>>24,df[a+4|0]=l,df[a+5|0]=l>>>8,df[a+6|0]=l>>>16,df[a+7|0]=l>>>24,n=n+8|0,(a=a+8|0)>>>0<h>>>0;);if(df[0|h]=A,df[h+1|0]=A>>>8,uf>>>0<6+((n=h+2|0)+(((b=p-4|0)>>>0)/255|0)|0)>>>0)break a;a=gf[0|_],15<=b>>>0?(df[0|_]=a+15,510<=(a=p-19|0)>>>0&&(If(A=n,255,(b=(a=((n=p-529|0)>>>0)/510|0)<<1)+2|0),a=n+Bf(a,-510)|0,n=4+((b+c|0)+s|0)|0),255<=a>>>0&&(df[0|n]=255,n=n+1|0,a=a-255|0),df[0|n]=a,n=n+1|0):df[0|_]=a+b,_=n,j=C=p+C|0}else u=u+1|0,C=C+1|0;if(!((0|u)<(0|V)))break}}}if(C>>>0<=ef>>>0)continue;break i}break}if(2!=((n=0)|k))break e}if(n=Q-j|0,a=(n+240>>>0)/255|0,k&&!((b=1+((a+n|0)+_|0)|0)>>>0<=(a=q?uf+5|0:$)>>>0)){if(1==((n=0)|k))break e;n=(a=a+(-1^_)|0)-((a+240>>>0)/255|0)|0}k=n+j|0,15<=n>>>0?(df[0|_]=240,a=_+1|0,(b=n-15|0)>>>0<255?df[0|(_=a)]=b:(If(u=a,255,(b=((a=n-270|0)>>>0)/255|0)+1|0),df[0|(_=2+(b+_|0)|0)]=a+Bf(b,-255))):df[0|_]=n<<4,a=Ef(_+1|0,j,n),wf[i>>2]=k-r,n=(a+n|0)-e|0}if(0<(0|n))break f}df[f+262171|0]=1}return _f=65600+S|0,n}function n(f,r,e,i,a,b){var k,o,t=0,n=0,s=0,u=0,c=0,l=0,A=0,h=0,p=0,m=0,y=0,d=0,v=0,w=0,g=0,S=0,B=0,_=0,Z=0,E=0,I=0,C=0,R=0,U=0,J=0;f:if(!Sf[f+16388>>1]){if(p=(t=wf[f+16400>>2])+(h=wf[f+16392>>2])|0,2147483649<=(m=wf[f+16384>>2])+i>>>0){for(t=m-65536|0;n=(l=wf[(c=(u=s<<2)+f|0)>>2])-t|0,wf[c>>2]=l>>>0<n>>>0?0:n,n=(l=wf[(c=(4|u)+f|0)>>2])-t|0,wf[c>>2]=l>>>0<n>>>0?0:n,n=(l=wf[(c=(8|u)+f|0)>>2])-t|0,wf[c>>2]=l>>>0<n>>>0?0:n,u=(c=wf[(n=(12|u)+f|0)>>2])-t|0,wf[n>>2]=c>>>0<u>>>0?0:u,4096!=(0|(s=s+4|0)););m=65536,wf[f+16384>>2]=65536,65537<=(t=wf[f+16400>>2])>>>0&&(t=wf[f+16400>>2]=65536),h=p-t|0,wf[f+16392>>2]=h}(0|r)==(0|p)|2<t-1>>>0||(p=h=wf[f+16392>>2]=r,t=wf[f+16400>>2]=0),s=1<(0|b),p>>>0<=(k=r+i|0)>>>0|k>>>0<=h>>>0||(t=(t=(t=p-k|0)>>>0<65536?t:65536)>>>0<4?0:t,h=p-(wf[f+16400>>2]=t)|0,wf[f+16392>>2]=h),S=s?b:1;r:{if((0|r)==(0|p)){if(v=r-m|0,65535<t>>>0|m>>>0<=t>>>0)break r;if(2113929216<i>>>0)break f;a=e+a|0,vf[f+16390>>1]=2,wf[f+16384>>2]=i+m,wf[f+16400>>2]=i+t;e:if((0|i)<13)h=e;else for(w=m-t|0,Z=k-11|0,y=r-t|0,n=r+1|0,A=(wf[(Bf(gf[0|r]|gf[r+1|0]<<8|(gf[r+2|0]<<16|gf[r+3|0]<<24),-1640531535)>>>18&16380)+f>>2]=m)+1|0,b=r+2|0,m=(d=k-5|0)-1|0,s=d-3|0,g=1|(c=S<<6),h=e;;){for(p=gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24),l=c,t=g;;){if(u=n,i=t,S=(Bf(p,-1640531535)>>>18&16380)+f|0,t=wf[S>>2],n=b,p=gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24),wf[S>>2]=A,t>>>0<w>>>0|t+65535>>>0<A>>>0||(gf[0|(A=t+v|0)]|gf[A+1|0]<<8|(gf[A+2|0]<<16|gf[A+3|0]<<24))!=(gf[0|u]|gf[u+1|0]<<8|(gf[u+2|0]<<16|gf[u+3|0]<<24))){if(b=l>>6,t=i+1|0,A=n-v|0,l=i,(b=b+n|0)>>>0<=Z>>>0)continue;break e}break}for(;!((i=u)>>>0<=r>>>0|(t=A)>>>0<=y>>>0||(A=t-1|0,gf[0|(u=i-1|0)]!=gf[0|A])););if(a>>>0<9+(((b=i-r|0)+h|0)+((b>>>0)/255|0)|0)>>>0)return 0;for(n=h+1|0,15<=b>>>0?(df[0|h]=240,255<=(0|(A=b-15|0))&&(If(n,255,(u=(239+(i-(((0|A)<509?A:509)+r|0)|0)>>>0)/255|0)+1|0),A=(b+Bf(u,-255)|0)-270|0,n=2+(u+h|0)|0),df[0|n]=A,n=n+1|0):df[0|h]=b<<4,A=b+n|0;b=gf[r+4|0]|gf[r+5|0]<<8|(gf[r+6|0]<<16|gf[r+7|0]<<24),u=gf[0|r]|gf[r+1|0]<<8|(gf[r+2|0]<<16|gf[r+3|0]<<24),df[0|n]=u,df[n+1|0]=u>>>8,df[n+2|0]=u>>>16,df[n+3|0]=u>>>24,df[n+4|0]=b,df[n+5|0]=b>>>8,df[n+6|0]=b>>>16,df[n+7|0]=b>>>24,r=r+8|0,(n=n+8|0)>>>0<A>>>0;);for(r=i;;){i=r-t|0,df[0|A]=i,df[A+1|0]=i>>>8,n=t+4|0;i:{a:{if((b=s)>>>0<=(i=r+4|0)>>>0)t=i;else{if(n=(gf[0|i]|gf[i+1|0]<<8|(gf[i+2|0]<<16|gf[i+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24)))break a;n=t+8|0,t=r+8|0}if(t>>>0<b>>>0)for(;;){if(l=(gf[0|t]|gf[t+1|0]<<8|(gf[t+2|0]<<16|gf[t+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))){i=((Rf(l)>>>3|0)+t|0)-i|0;break i}if(n=n+4|0,!((t=t+4|0)>>>0<s>>>0))break}(gf[0|n]|gf[n+1|0]<<8)!=(gf[0|t]|gf[t+1|0]<<8)|m>>>0<=t>>>0||(n=n+2|0,t=t+2|0),t>>>0<d>>>0&&(t=gf[0|n]==gf[0|t]?t+1|0:t),i=t-i|0;break i}i=Rf(n)>>>3|0}if(a>>>0<8+(((i+240>>>0)/255|0)+A|0)>>>0)return 0;if(b=h,h=A+2|0,r=4+(r+i|0)|0,t=gf[0|b],15<=i>>>0?(df[0|b]=t+15,df[0|h]=255,df[h+1|0]=255,df[h+2|0]=255,df[h+3|0]=255,1020<=(t=i-15|0)>>>0&&(t=Bf(b=((i=i-1035|0)>>>0)/1020|0,-1020)+i|0,h=If(A+6|0,255,(i=b<<2)+4|0)+i|0),df[0|(b=h+(i=((65535&t)>>>0)/255|0)|0)]=i+t,h=b+1|0):df[0|b]=i+t,Z>>>0<=r>>>0)break e;if(wf[(Bf(gf[0|(i=r-2|0)]|gf[i+1|0]<<8|(gf[i+2|0]<<16|gf[i+3|0]<<24),-1640531535)>>>18&16380)+f>>2]=i-v,b=(Bf(gf[0|r]|gf[r+1|0]<<8|(gf[r+2|0]<<16|gf[r+3|0]<<24),-1640531535)>>>18&16380)+f|0,i=wf[b>>2],t=b,b=r-v|0,wf[t>>2]=b,i>>>0<w>>>0|i+65535>>>0<b>>>0||(gf[0|(t=i+v|0)]|gf[t+1|0]<<8|(gf[t+2|0]<<16|gf[t+3|0]<<24))!=(gf[0|r]|gf[r+1|0]<<8|(gf[r+2|0]<<16|gf[r+3|0]<<24)))break;A=h+1|(df[0|h]=0)}if(A=(n=r+1|0)-v|0,!((b=r+2|0)>>>0<=Z>>>0))break}if(a>>>0<1+(((i=k-r|0)+h|0)+((i+240>>>0)/255|0)|0)>>>0)break f;return 15<=i>>>0?(df[0|h]=240,f=h+1|0,(a=i-15|0)>>>0<255?df[0|(h=f)]=a:(If(b=f,255,(f=((a=i-270|0)>>>0)/255|0)+1|0),df[0|(h=2+(f+h|0)|0)]=a+Bf(f,-255))):df[0|h]=i<<4,(Ef(h+1|0,r,i)+i|0)-e|0}e:{i:{if(y=wf[f+16396>>2]){if((0|i)<4097)break i;if(w=Ef(f,y,16416),2113929216<i>>>0)break e;for(_=e+a|0,I=k-11|0,y=r-(d=wf[w+16384>>2])|0,C=(U=(E=wf[w+16392>>2])+(a=wf[w+16400>>2])|0)-d|0,vf[w+16390>>1]=2,wf[w+16384>>2]=i+d,wf[w+16400>>2]=i+a,u=r+1|0,t=(wf[w+(Bf(gf[0|r]|gf[r+1|0]<<8|(gf[r+2|0]<<16|gf[r+3|0]<<24),-1640531535)>>>18&16380)>>2]=d)+1|0,n=r+2|0,g=r+4|0,R=(B=k-5|0)-1|0,l=B-3|0,Z=1|(v=S<<6),s=r,h=e;;){m=gf[0|u]|gf[u+1|0]<<8|(gf[u+2|0]<<16|gf[u+3|0]<<24),a=v,A=Z;a:{for(;c=w+(Bf(m,-1640531535)>>>18&16380)|0,b=wf[c>>2],m=gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24),!((wf[c>>2]=t)>>>0<=b+65535>>>0&&(gf[0|(p=b+((c=b>>>0<d>>>0)?C:y)|0)]|gf[p+1|0]<<8|(gf[p+2|0]<<16|gf[p+3|0]<<24))==(gf[0|u]|gf[u+1|0]<<8|(gf[u+2|0]<<16|gf[u+3|0]<<24)));)if(b=a>>6,t=n-y|0,A=(a=A)+1|0,!((n=b+(u=n)|0)>>>0<=I>>>0))break a;for(m=c?E:r,c=t-b|0;!((a=u)>>>0<=s>>>0|(t=p)>>>0<=m>>>0||(p=t-1|0,gf[0|(u=a-1|0)]!=gf[0|p])););if(_>>>0<9+(((b=a-s|0)+h|0)+((b>>>0)/255|0)|0)>>>0)break e;for(n=h+1|0,15<=b>>>0?(df[0|h]=240,255<=(0|(p=b-15|0))&&(If(n,255,(u=(239+(a-(((0|p)<509?p:509)+s|0)|0)>>>0)/255|0)+1|0),p=(b+Bf(u,-255)|0)-270|0,n=2+(u+h|0)|0),df[0|n]=p,n=n+1|0):df[0|h]=b<<4,A=b+n|0;b=gf[s+4|0]|gf[s+5|0]<<8|(gf[s+6|0]<<16|gf[s+7|0]<<24),u=gf[0|s]|gf[s+1|0]<<8|(gf[s+2|0]<<16|gf[s+3|0]<<24),df[0|n]=u,df[n+1|0]=u>>>8,df[n+2|0]=u>>>16,df[n+3|0]=u>>>24,df[n+4|0]=b,df[n+5|0]=b>>>8,df[n+6|0]=b>>>16,df[n+7|0]=b>>>24,s=s+8|0,(n=n+8|0)>>>0<A>>>0;);for(s=a;;){df[0|A]=c,df[A+1|0]=c>>>8,b=s;b:{k:{o:{t:{n:{if((0|m)==(0|E)){if(n=t+4|0,(u=(a=B>>>0<(a=(U-t|0)+s|0)>>>0?B:a)-3|0)>>>0<=(b=s+4|0)>>>0)t=b;else{if(n=(gf[0|b]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24)))break n;n=t+8|0,t=s+8|0}if(t>>>0<u>>>0)for(;;){if(c=(gf[0|t]|gf[t+1|0]<<8|(gf[t+2|0]<<16|gf[t+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))){n=((Rf(c)>>>3|0)+t|0)-b|0;break k}if(n=n+4|0,!((t=t+4|0)>>>0<u>>>0))break}(gf[0|n]|gf[n+1|0]<<8)!=(gf[0|t]|gf[t+1|0]<<8)|a-1>>>0<=t>>>0||(n=n+2|0,t=t+2|0),t>>>0<a>>>0&&(t=gf[0|n]==gf[0|t]?t+1|0:t),n=t-b|0;break k}if(n=t+4|0,(u=l)>>>0<=(a=s+4|0)>>>0)t=a;else{if(n=(gf[0|a]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24)))break t;n=t+8|0,t=s+8|0}if(t>>>0<u>>>0)for(;;){if(s=(gf[0|t]|gf[t+1|0]<<8|(gf[t+2|0]<<16|gf[t+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))){n=((Rf(s)>>>3|0)+t|0)-a|0;break o}if(n=n+4|0,!((t=t+4|0)>>>0<l>>>0))break}(gf[0|n]|gf[n+1|0]<<8)!=(gf[0|t]|gf[t+1|0]<<8)|R>>>0<=t>>>0||(n=n+2|0,t=t+2|0),t>>>0<B>>>0&&(t=gf[0|n]==gf[0|t]?t+1|0:t),n=t-a|0;break o}n=Rf(n)>>>3|0;break k}n=Rf(n)>>>3|0}s=4+(n+b|0)|0;break b}if((0|a)==(0|(s=4+(n+s|0)|0))){s=r;k:{o:{if((t=b=a)>>>0<l>>>0){if(t=(gf[0|a]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24))^(gf[0|r]|gf[r+1|0]<<8|(gf[r+2|0]<<16|gf[r+3|0]<<24)))break o;s=g,t=a+4|0}if(t>>>0<l>>>0)for(;;){if(u=(gf[0|t]|gf[t+1|0]<<8|(gf[t+2|0]<<16|gf[t+3|0]<<24))^(gf[0|s]|gf[s+1|0]<<8|(gf[s+2|0]<<16|gf[s+3|0]<<24))){a=((Rf(u)>>>3|0)+t|0)-a|0;break k}if(s=s+4|0,!((t=t+4|0)>>>0<l>>>0))break}(gf[0|s]|gf[s+1|0]<<8)!=(gf[0|t]|gf[t+1|0]<<8)|R>>>0<=t>>>0||(s=s+2|0,t=t+2|0),t>>>0<B>>>0&&(t=gf[0|s]==gf[0|t]?t+1|0:t),a=t-a|0;break k}a=Rf(t)>>>3|0}s=b+a|0,n=a+n|0}}if(_>>>0<8+(((n+240>>>0)/255|0)+A|0)>>>0)break e;if(a=h,h=A+2|0,b=gf[0|a],15<=n>>>0?(df[0|a]=b+15,df[0|h]=255,df[h+1|0]=255,df[h+2|0]=255,df[h+3|0]=255,1020<=(t=n-15|0)>>>0&&(t=Bf(b=((a=n-1035|0)>>>0)/1020|0,-1020)+a|0,h=If(A+6|0,255,(a=b<<2)+4|0)+a|0),df[0|(b=h+(a=((65535&t)>>>0)/255|0)|0)]=a+t,h=b+1|0):df[0|a]=b+n,I>>>0<=s>>>0)break a;if(wf[w+(Bf(gf[0|(a=s-2|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24),-1640531535)>>>18&16380)>>2]=a-y,b=w+(Bf(gf[0|s]|gf[s+1|0]<<8|(gf[s+2|0]<<16|gf[s+3|0]<<24),-1640531535)>>>18&16380)|0,a=wf[b>>2],t=b,b=s-y|0,wf[t>>2]=b,a+65535>>>0<b>>>0||(gf[0|(t=a+((u=a>>>0<d>>>0)?C:y)|0)]|gf[t+1|0]<<8|(gf[t+2|0]<<16|gf[t+3|0]<<24))!=(gf[0|s]|gf[s+1|0]<<8|(gf[s+2|0]<<16|gf[s+3|0]<<24)))break;m=u?E:r,c=b-a|(df[0|h]=0),A=h+1|0}if(t=(u=s+1|0)-y|0,(n=s+2|0)>>>0<=I>>>0)continue}break}if(_>>>0<1+(((b=k-s|0)+h|0)+((b+240>>>0)/255|0)|0)>>>0)break e;15<=b>>>0?(df[0|h]=240,a=h+1|0,(t=b-15|0)>>>0<255?df[0|(h=a)]=t:(If(u=a,255,(a=((t=b-270|0)>>>0)/255|0)+1|0),df[0|(h=2+(a+h|0)|0)]=t+Bf(a,-255))):df[0|h]=b<<4,J=(Ef(h+1|0,s,b)+b|0)-e|0;break e}if(d=r-m|0,b=0-m|0,!(65535<t>>>0|m>>>0<=t>>>0)){if(2113929216<i>>>0)break e;B=e+a|0,vf[f+16390>>1]=2,wf[f+16384>>2]=i+m,wf[f+16400>>2]=i+t,l=e,c=r;a:if(!((0|i)<13))for(I=m-t|0,E=(R=t+h|0)+b|0,_=k-11|0,t=r+1|0,p=(wf[(Bf(gf[0|r]|gf[r+1|0]<<8|(gf[r+2|0]<<16|gf[r+3|0]<<24),-1640531535)>>>18&16380)+f>>2]=m)+1|0,a=r+2|0,v=r+4|0,C=(y=k-5|0)-1|0,g=y-3|0,w=1|(Z=S<<6);;){for(b=gf[0|t]|gf[t+1|0]<<8|(gf[t+2|0]<<16|gf[t+3|0]<<24),u=Z,s=w;;){if(n=t,A=s,S=(Bf(b,-1640531535)>>>18&16380)+f|0,s=wf[S>>2],t=a,b=gf[0|t]|gf[t+1|0]<<8|(gf[t+2|0]<<16|gf[t+3|0]<<24),wf[S>>2]=p,s>>>0<I>>>0|s+65535>>>0<p>>>0||(gf[0|(a=s+((S=s>>>0<m>>>0)?E:d)|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24))!=(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))){if(a=u>>6,s=A+1|0,p=t-d|0,u=A,(a=a+t|0)>>>0<=_>>>0)continue;break a}break}for(b=S?h:r,A=p-s|0;!((u=n)>>>0<=c>>>0|(s=a)>>>0<=b>>>0||(a=s-1|0,gf[0|(n=u-1|0)]!=gf[0|a])););if(B>>>0<9+(((n=u-c|0)+l|0)+((n>>>0)/255|0)|0)>>>0)break e;for(t=l+1|0,15<=n>>>0?(df[0|l]=240,255<=(0|(a=n-15|0))&&(If(p=t,255,(t=(239+(u-(((0|a)<509?a:509)+c|0)|0)>>>0)/255|0)+1|0),a=(n+Bf(t,-255)|0)-270|0,t=2+(t+l|0)|0),df[0|t]=a,t=t+1|0):df[0|l]=n<<4,p=t+n|0;a=gf[c+4|0]|gf[c+5|0]<<8|(gf[c+6|0]<<16|gf[c+7|0]<<24),n=gf[0|c]|gf[c+1|0]<<8|(gf[c+2|0]<<16|gf[c+3|0]<<24),df[0|t]=n,df[t+1|0]=n>>>8,df[t+2|0]=n>>>16,df[t+3|0]=n>>>24,df[t+4|0]=a,df[t+5|0]=a>>>8,df[t+6|0]=a>>>16,df[t+7|0]=a>>>24,c=c+8|0,(t=t+8|0)>>>0<p>>>0;);for(c=u;;){df[0|p]=A,df[p+1|0]=A>>>8,u=c;b:{k:{o:{t:{n:{if((0|b)==(0|h)){if(t=s+4|0,(u=(a=y>>>0<(a=(R-s|0)+c|0)>>>0?y:a)-3|0)>>>0<=(b=c+4|0)>>>0)s=b;else{if(t=(gf[0|b]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24))^(gf[0|t]|gf[t+1|0]<<8|(gf[t+2|0]<<16|gf[t+3|0]<<24)))break n;t=s+8|0,s=c+8|0}if(s>>>0<u>>>0)for(;;){if(n=(gf[0|s]|gf[s+1|0]<<8|(gf[s+2|0]<<16|gf[s+3|0]<<24))^(gf[0|t]|gf[t+1|0]<<8|(gf[t+2|0]<<16|gf[t+3|0]<<24))){t=((Rf(n)>>>3|0)+s|0)-b|0;break k}if(t=t+4|0,!((s=s+4|0)>>>0<u>>>0))break}(gf[0|t]|gf[t+1|0]<<8)!=(gf[0|s]|gf[s+1|0]<<8)|a-1>>>0<=s>>>0||(s=s+2|0,t=t+2|0),s>>>0<a>>>0&&(s=gf[0|t]==gf[0|s]?s+1|0:s),t=s-b|0;break k}if(t=s+4|0,(b=g)>>>0<=(a=c+4|0)>>>0)s=a;else{if(t=(gf[0|a]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24))^(gf[0|t]|gf[t+1|0]<<8|(gf[t+2|0]<<16|gf[t+3|0]<<24)))break t;t=s+8|0,s=c+8|0}if(s>>>0<b>>>0)for(;;){if(b=(gf[0|s]|gf[s+1|0]<<8|(gf[s+2|0]<<16|gf[s+3|0]<<24))^(gf[0|t]|gf[t+1|0]<<8|(gf[t+2|0]<<16|gf[t+3|0]<<24))){t=((Rf(b)>>>3|0)+s|0)-a|0;break o}if(t=t+4|0,!((s=s+4|0)>>>0<g>>>0))break}(gf[0|t]|gf[t+1|0]<<8)!=(gf[0|s]|gf[s+1|0]<<8)|C>>>0<=s>>>0||(s=s+2|0,t=t+2|0),s>>>0<y>>>0&&(s=gf[0|t]==gf[0|s]?s+1|0:s),t=s-a|0;break o}t=Rf(t)>>>3|0;break k}t=Rf(t)>>>3|0}c=4+(t+u|0)|0;break b}if((0|a)==(0|(c=4+(t+c|0)|0))){c=r;k:{o:{if((s=b=a)>>>0<g>>>0){if(s=(gf[0|a]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24))^(gf[0|r]|gf[r+1|0]<<8|(gf[r+2|0]<<16|gf[r+3|0]<<24)))break o;c=v,s=a+4|0}if(s>>>0<g>>>0)for(;;){if(u=(gf[0|s]|gf[s+1|0]<<8|(gf[s+2|0]<<16|gf[s+3|0]<<24))^(gf[0|c]|gf[c+1|0]<<8|(gf[c+2|0]<<16|gf[c+3|0]<<24))){a=((Rf(u)>>>3|0)+s|0)-a|0;break k}if(c=c+4|0,!((s=s+4|0)>>>0<g>>>0))break}(gf[0|c]|gf[c+1|0]<<8)!=(gf[0|s]|gf[s+1|0]<<8)|C>>>0<=s>>>0||(c=c+2|0,s=s+2|0),s>>>0<y>>>0&&(s=gf[0|c]==gf[0|s]?s+1|0:s),a=s-a|0;break k}a=Rf(s)>>>3|0}c=b+a|0,t=a+t|0}}if(B>>>0<8+(((t+240>>>0)/255|0)+p|0)>>>0)break e;if(a=l,l=p+2|0,b=gf[0|a],15<=t>>>0?(df[0|a]=b+15,df[0|l]=255,df[l+1|0]=255,df[l+2|0]=255,df[l+3|0]=255,1020<=(s=t-15|0)>>>0&&(s=Bf(b=((a=t-1035|0)>>>0)/1020|0,-1020)+a|0,l=If(p+6|0,255,(a=b<<2)+4|0)+a|0),df[0|(b=l+(a=((65535&s)>>>0)/255|0)|0)]=a+s,l=b+1|0):df[0|a]=b+t,_>>>0<=c>>>0)break a;if(wf[(Bf(gf[0|(a=c-2|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24),-1640531535)>>>18&16380)+f>>2]=a-d,b=(Bf(gf[0|c]|gf[c+1|0]<<8|(gf[c+2|0]<<16|gf[c+3|0]<<24),-1640531535)>>>18&16380)+f|0,a=wf[b>>2],t=c-d|0,wf[b>>2]=t,a>>>0<I>>>0|a+65535>>>0<t>>>0||(gf[0|(s=a+((b=a>>>0<m>>>0)?E:d)|0)]|gf[s+1|0]<<8|(gf[s+2|0]<<16|gf[s+3|0]<<24))!=(gf[0|c]|gf[c+1|0]<<8|(gf[c+2|0]<<16|gf[c+3|0]<<24)))break;b=b?h:r,A=t-a|(df[0|l]=0),p=l+1|0}if(p=(t=c+1|0)-d|0,!((a=c+2|0)>>>0<=_>>>0))break}if(B>>>0<1+(((b=k-c|0)+l|0)+((b+240>>>0)/255|0)|0)>>>0)break e;15<=b>>>0?(df[0|l]=240,a=l+1|0,(t=b-15|0)>>>0<255?df[0|(l=a)]=t:(If(s=a,255,(a=((t=b-270|0)>>>0)/255|0)+1|0),df[0|(l=2+(a+l|0)|0)]=t+Bf(a,-255))):df[0|l]=b<<4,J=(Ef(l+1|0,c,b)+b|0)-e|0;break e}if(2113929216<i>>>0)break e;B=e+a|0,vf[f+16390>>1]=2,wf[f+16384>>2]=i+m,wf[f+16400>>2]=i+t,n=e,c=r;a:if(!((0|i)<13))for(I=(C=t+h|0)+b|0,_=k-11|0,u=r+1|0,s=(wf[(Bf(gf[0|r]|gf[r+1|0]<<8|(gf[r+2|0]<<16|gf[r+3|0]<<24),-1640531535)>>>18&16380)+f>>2]=m)+1|0,t=r+2|0,v=r+4|0,E=(y=k-5|0)-1|0,g=y-3|0,w=1|(Z=S<<6);;){for(b=gf[0|u]|gf[u+1|0]<<8|(gf[u+2|0]<<16|gf[u+3|0]<<24),l=Z,p=w;a=(Bf(b,-1640531535)>>>18&16380)+f|0,A=wf[a>>2],b=gf[0|t]|gf[t+1|0]<<8|(gf[t+2|0]<<16|gf[t+3|0]<<24),!((wf[a>>2]=s)>>>0<=A+65535>>>0&&(gf[0|(a=A+((S=A>>>0<m>>>0)?I:d)|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24))==(gf[0|u]|gf[u+1|0]<<8|(gf[u+2|0]<<16|gf[u+3|0]<<24)));)if(a=l>>6,s=t-d|0,p=(l=p)+1|0,!((t=a+(u=t)|0)>>>0<=_>>>0))break a;for(b=S?h:r,A=s-A|0;!((l=u)>>>0<=c>>>0|(s=a)>>>0<=b>>>0||(a=s-1|0,gf[0|(u=l-1|0)]!=gf[0|a])););if(B>>>0<9+(((u=l-c|0)+n|0)+((u>>>0)/255|0)|0)>>>0)break e;for(t=n+1|0,15<=u>>>0?(df[0|n]=240,255<=(0|(a=u-15|0))&&(If(p=t,255,(t=(239+(l-(((0|a)<509?a:509)+c|0)|0)>>>0)/255|0)+1|0),a=(u+Bf(t,-255)|0)-270|0,t=2+(t+n|0)|0),df[0|t]=a,t=t+1|0):df[0|n]=u<<4,p=t+u|0;a=gf[c+4|0]|gf[c+5|0]<<8|(gf[c+6|0]<<16|gf[c+7|0]<<24),u=gf[0|c]|gf[c+1|0]<<8|(gf[c+2|0]<<16|gf[c+3|0]<<24),df[0|t]=u,df[t+1|0]=u>>>8,df[t+2|0]=u>>>16,df[t+3|0]=u>>>24,df[t+4|0]=a,df[t+5|0]=a>>>8,df[t+6|0]=a>>>16,df[t+7|0]=a>>>24,c=c+8|0,(t=t+8|0)>>>0<p>>>0;);for(c=l;;){df[0|p]=A,df[p+1|0]=A>>>8,u=c;b:{k:{o:{t:{n:{if((0|b)==(0|h)){if(t=s+4|0,(u=(a=y>>>0<(a=(C-s|0)+c|0)>>>0?y:a)-3|0)>>>0<=(b=c+4|0)>>>0)s=b;else{if(t=(gf[0|b]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24))^(gf[0|t]|gf[t+1|0]<<8|(gf[t+2|0]<<16|gf[t+3|0]<<24)))break n;t=s+8|0,s=c+8|0}if(s>>>0<u>>>0)for(;;){if(l=(gf[0|s]|gf[s+1|0]<<8|(gf[s+2|0]<<16|gf[s+3|0]<<24))^(gf[0|t]|gf[t+1|0]<<8|(gf[t+2|0]<<16|gf[t+3|0]<<24))){t=((Rf(l)>>>3|0)+s|0)-b|0;break k}if(t=t+4|0,!((s=s+4|0)>>>0<u>>>0))break}(gf[0|t]|gf[t+1|0]<<8)!=(gf[0|s]|gf[s+1|0]<<8)|a-1>>>0<=s>>>0||(s=s+2|0,t=t+2|0),s>>>0<a>>>0&&(s=gf[0|t]==gf[0|s]?s+1|0:s),t=s-b|0;break k}if(t=s+4|0,(b=g)>>>0<=(a=c+4|0)>>>0)s=a;else{if(t=(gf[0|a]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24))^(gf[0|t]|gf[t+1|0]<<8|(gf[t+2|0]<<16|gf[t+3|0]<<24)))break t;t=s+8|0,s=c+8|0}if(s>>>0<b>>>0)for(;;){if(b=(gf[0|s]|gf[s+1|0]<<8|(gf[s+2|0]<<16|gf[s+3|0]<<24))^(gf[0|t]|gf[t+1|0]<<8|(gf[t+2|0]<<16|gf[t+3|0]<<24))){t=((Rf(b)>>>3|0)+s|0)-a|0;break o}if(t=t+4|0,!((s=s+4|0)>>>0<g>>>0))break}(gf[0|t]|gf[t+1|0]<<8)!=(gf[0|s]|gf[s+1|0]<<8)|E>>>0<=s>>>0||(s=s+2|0,t=t+2|0),s>>>0<y>>>0&&(s=gf[0|t]==gf[0|s]?s+1|0:s),t=s-a|0;break o}t=Rf(t)>>>3|0;break k}t=Rf(t)>>>3|0}c=4+(t+u|0)|0;break b}if((0|a)==(0|(c=4+(t+c|0)|0))){c=r;k:{o:{if((s=b=a)>>>0<g>>>0){if(s=(gf[0|a]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24))^(gf[0|r]|gf[r+1|0]<<8|(gf[r+2|0]<<16|gf[r+3|0]<<24)))break o;c=v,s=a+4|0}if(s>>>0<g>>>0)for(;;){if(u=(gf[0|s]|gf[s+1|0]<<8|(gf[s+2|0]<<16|gf[s+3|0]<<24))^(gf[0|c]|gf[c+1|0]<<8|(gf[c+2|0]<<16|gf[c+3|0]<<24))){a=((Rf(u)>>>3|0)+s|0)-a|0;break k}if(c=c+4|0,!((s=s+4|0)>>>0<g>>>0))break}(gf[0|c]|gf[c+1|0]<<8)!=(gf[0|s]|gf[s+1|0]<<8)|E>>>0<=s>>>0||(c=c+2|0,s=s+2|0),s>>>0<y>>>0&&(s=gf[0|c]==gf[0|s]?s+1|0:s),a=s-a|0;break k}a=Rf(s)>>>3|0}c=b+a|0,t=a+t|0}}if(B>>>0<8+(((t+240>>>0)/255|0)+p|0)>>>0)break e;if(a=n,n=p+2|0,b=gf[0|a],15<=t>>>0?(df[0|a]=b+15,df[0|n]=255,df[n+1|0]=255,df[n+2|0]=255,df[n+3|0]=255,1020<=(s=t-15|0)>>>0&&(s=Bf(b=((a=t-1035|0)>>>0)/1020|0,-1020)+a|0,n=If(p+6|0,255,(a=b<<2)+4|0)+a|0),df[0|(b=n+(a=((65535&s)>>>0)/255|0)|0)]=a+s,n=b+1|0):df[0|a]=b+t,_>>>0<=c>>>0)break a;if(wf[(Bf(gf[0|(a=c-2|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24),-1640531535)>>>18&16380)+f>>2]=a-d,b=(Bf(gf[0|c]|gf[c+1|0]<<8|(gf[c+2|0]<<16|gf[c+3|0]<<24),-1640531535)>>>18&16380)+f|0,a=wf[b>>2],t=c-d|0,wf[b>>2]=t,a+65535>>>0<t>>>0||(gf[0|(s=a+((b=a>>>0<m>>>0)?I:d)|0)]|gf[s+1|0]<<8|(gf[s+2|0]<<16|gf[s+3|0]<<24))!=(gf[0|c]|gf[c+1|0]<<8|(gf[c+2|0]<<16|gf[c+3|0]<<24)))break;b=b?h:r,A=t-a|(df[0|n]=0),p=n+1|0}if(s=(u=c+1|0)-d|0,!((t=c+2|0)>>>0<=_>>>0))break}if(B>>>0<1+(((b=k-c|0)+n|0)+((b+240>>>0)/255|0)|0)>>>0)break e;15<=b>>>0?(df[0|n]=240,a=n+1|0,(t=b-15|0)>>>0<255?df[0|(n=a)]=t:(If(s=a,255,(a=((t=b-270|0)>>>0)/255|0)+1|0),df[0|(n=2+(a+n|0)|0)]=t+Bf(a,-255))):df[0|n]=b<<4,J=(Ef(n+1|0,c,b)+b|0)-e|0;break e}if(!(2113929216<i>>>0)){I=e+a|0,a=wf[y+16384>>2],b=wf[y+16400>>2],g=wf[y+16392>>2],wf[f+16400>>2]=i,wf[f+16396>>2]=0,vf[f+16390>>1]=2,wf[f+16384>>2]=i+m,l=e,s=r;i:if(!((0|i)<13))for(B=r-m|0,E=k-11|0,C=m-a|0,R=(o=b+g|0)-a|0,u=r+1|0,t=(wf[(Bf(gf[0|r]|gf[r+1|0]<<8|(gf[r+2|0]<<16|gf[r+3|0]<<24),-1640531535)>>>18&16380)+f>>2]=m)+1|0,n=r+2|0,Z=r+4|0,U=(_=k-5|0)-1|0,v=_-3|0,d=1|(w=S<<6);;){for(p=gf[0|u]|gf[u+1|0]<<8|(gf[u+2|0]<<16|gf[u+3|0]<<24),a=w,A=d;b=n,n=Bf(p,-1640531535)>>>20<<2,n=m>>>0<=(c=wf[(S=n+f|0)>>2])>>>0?(h=r,c+B|0):(c=(n=wf[n+y>>2])+C|0,h=g,n+R|0),p=gf[0|b]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24),wf[S>>2]=t,!((gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))==(gf[0|u]|gf[u+1|0]<<8|(gf[u+2|0]<<16|gf[u+3|0]<<24))&&t>>>0<=c+65535>>>0);)if(n=a>>6,t=b-B|0,A=(a=A)+1|0,!((n=n+(u=b)|0)>>>0<=E>>>0))break i;for(c=t-c|0;!((a=u)>>>0<=s>>>0|(t=n)>>>0<=h>>>0||(n=t-1|0,gf[0|(u=a-1|0)]!=gf[0|n])););if(I>>>0<9+(((u=a-s|0)+l|0)+((u>>>0)/255|0)|0)>>>0)break e;for(n=l+1|0,15<=u>>>0?(df[0|l]=240,255<=(0|(b=u-15|0))&&(If(n,255,(b=(239+(a-(((0|b)<509?b:509)+s|0)|0)>>>0)/255|0)+1|0),n=2+(b+l|0)|0,b=(u+Bf(b,-255)|0)-270|0),df[0|n]=b,n=n+1|0):df[0|l]=u<<4,A=n+u|0;b=gf[s+4|0]|gf[s+5|0]<<8|(gf[s+6|0]<<16|gf[s+7|0]<<24),u=gf[0|s]|gf[s+1|0]<<8|(gf[s+2|0]<<16|gf[s+3|0]<<24),df[0|n]=u,df[n+1|0]=u>>>8,df[n+2|0]=u>>>16,df[n+3|0]=u>>>24,df[n+4|0]=b,df[n+5|0]=b>>>8,df[n+6|0]=b>>>16,df[n+7|0]=b>>>24,s=s+8|0,(n=n+8|0)>>>0<A>>>0;);for(s=a;;){df[0|A]=c,df[A+1|0]=c>>>8,b=s;a:{b:{k:{o:{t:{if((0|h)==(0|g)){if(n=t+4|0,(u=(a=_>>>0<(a=(o-t|0)+s|0)>>>0?_:a)-3|0)>>>0<=(b=s+4|0)>>>0)t=b;else{if(n=(gf[0|b]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24)))break t;n=t+8|0,t=s+8|0}if(t>>>0<u>>>0)for(;;){if(c=(gf[0|t]|gf[t+1|0]<<8|(gf[t+2|0]<<16|gf[t+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))){n=((Rf(c)>>>3|0)+t|0)-b|0;break b}if(n=n+4|0,!((t=t+4|0)>>>0<u>>>0))break}(gf[0|n]|gf[n+1|0]<<8)!=(gf[0|t]|gf[t+1|0]<<8)|a-1>>>0<=t>>>0||(n=n+2|0,t=t+2|0),t>>>0<a>>>0&&(t=gf[0|n]==gf[0|t]?t+1|0:t),n=t-b|0;break b}if(n=t+4|0,(u=v)>>>0<=(a=s+4|0)>>>0)t=a;else{if(n=(gf[0|a]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24)))break o;n=t+8|0,t=s+8|0}if(t>>>0<u>>>0)for(;;){if(s=(gf[0|t]|gf[t+1|0]<<8|(gf[t+2|0]<<16|gf[t+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))){n=((Rf(s)>>>3|0)+t|0)-a|0;break k}if(n=n+4|0,!((t=t+4|0)>>>0<v>>>0))break}(gf[0|n]|gf[n+1|0]<<8)!=(gf[0|t]|gf[t+1|0]<<8)|U>>>0<=t>>>0||(n=n+2|0,t=t+2|0),t>>>0<_>>>0&&(t=gf[0|n]==gf[0|t]?t+1|0:t),n=t-a|0;break k}n=Rf(n)>>>3|0;break b}n=Rf(n)>>>3|0}s=4+(n+b|0)|0;break a}if((0|a)==(0|(s=4+(n+s|0)|0))){s=r;b:{k:{if((t=b=a)>>>0<v>>>0){if(t=(gf[0|a]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24))^(gf[0|r]|gf[r+1|0]<<8|(gf[r+2|0]<<16|gf[r+3|0]<<24)))break k;s=Z,t=a+4|0}if(t>>>0<v>>>0)for(;;){if(u=(gf[0|t]|gf[t+1|0]<<8|(gf[t+2|0]<<16|gf[t+3|0]<<24))^(gf[0|s]|gf[s+1|0]<<8|(gf[s+2|0]<<16|gf[s+3|0]<<24))){a=((Rf(u)>>>3|0)+t|0)-a|0;break b}if(s=s+4|0,!((t=t+4|0)>>>0<v>>>0))break}(gf[0|s]|gf[s+1|0]<<8)!=(gf[0|t]|gf[t+1|0]<<8)|U>>>0<=t>>>0||(s=s+2|0,t=t+2|0),t>>>0<_>>>0&&(t=gf[0|s]==gf[0|t]?t+1|0:t),a=t-a|0;break b}a=Rf(t)>>>3|0}s=b+a|0,n=a+n|0}}if(I>>>0<8+(((n+240>>>0)/255|0)+A|0)>>>0)break e;if(a=l,l=A+2|0,b=gf[0|a],15<=n>>>0?(df[0|a]=b+15,df[0|l]=255,df[l+1|0]=255,df[l+2|0]=255,df[l+3|0]=255,1020<=(t=n-15|0)>>>0&&(t=Bf(b=((a=n-1035|0)>>>0)/1020|0,-1020)+a|0,l=If(A+6|0,255,(a=b<<2)+4|0)+a|0),df[0|(b=l+(a=((65535&t)>>>0)/255|0)|0)]=a+t,l=b+1|0):df[0|a]=b+n,E>>>0<=s>>>0)break i;if(wf[(Bf(gf[0|(a=s-2|0)]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24),-1640531535)>>>18&16380)+f>>2]=a-B,a=s-B|0,b=Bf(gf[0|s]|gf[s+1|0]<<8|(gf[s+2|0]<<16|gf[s+3|0]<<24),-1640531535)>>>20<<2,t=(n=wf[(u=b+f|0)>>2])>>>0<m>>>0?(n=(b=wf[b+y>>2])+C|0,h=g,b+R|0):(h=r,n+B|0),wf[u>>2]=a,(gf[0|t]|gf[t+1|0]<<8|(gf[t+2|0]<<16|gf[t+3|0]<<24))!=(gf[0|s]|gf[s+1|0]<<8|(gf[s+2|0]<<16|gf[s+3|0]<<24))|n+65535>>>0<a>>>0)break;c=a-n|(df[0|l]=0),A=l+1|0}if(t=(u=s+1|0)-B|0,!((n=s+2|0)>>>0<=E>>>0))break}I>>>0<1+(((b=k-s|0)+l|0)+((b+240>>>0)/255|0)|0)>>>0||(15<=b>>>0?(df[0|l]=240,a=l+1|0,(t=b-15|0)>>>0<255?df[0|(l=a)]=t:(If(u=a,255,(a=((t=b-270|0)>>>0)/255|0)+1|0),df[0|(l=2+(a+l|0)|0)]=t+Bf(a,-255))):df[0|l]=b<<4,J=(Ef(l+1|0,s,b)+b|0)-e|0)}}wf[f+16400>>2]=i,wf[f+16392>>2]=r;break f}if(!(2113929216<i>>>0)){b=e+a|0,vf[f+16390>>1]=2,wf[f+16384>>2]=i+m,wf[f+16400>>2]=i+t;r:if((0|i)<13)u=e;else for(Z=k-11|0,d=r-t|0,l=r+1|0,t=(wf[(Bf(gf[0|r]|gf[r+1|0]<<8|(gf[r+2|0]<<16|gf[r+3|0]<<24),-1640531535)>>>18&16380)+f>>2]=m)+1|0,n=r+2|0,m=(w=k-5|0)-1|0,s=w-3|0,g=1|(h=S<<6),u=e;;){for(p=gf[0|l]|gf[l+1|0]<<8|(gf[l+2|0]<<16|gf[l+3|0]<<24),a=h,c=g;A=(Bf(p,-1640531535)>>>18&16380)+f|0,i=wf[A>>2],p=gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24),!((wf[A>>2]=t)>>>0<=i+65535>>>0&&(gf[0|(A=i+v|0)]|gf[A+1|0]<<8|(gf[A+2|0]<<16|gf[A+3|0]<<24))==(gf[0|l]|gf[l+1|0]<<8|(gf[l+2|0]<<16|gf[l+3|0]<<24)));)if(i=a>>6,t=n-v|0,c=(a=c)+1|0,!((n=i+(l=n)|0)>>>0<=Z>>>0))break r;for(;!((i=l)>>>0<=r>>>0|(t=A)>>>0<=d>>>0||(A=t-1|0,gf[0|(l=i-1|0)]!=gf[0|A])););if(b>>>0<9+(((a=i-r|0)+u|0)+((a>>>0)/255|0)|0)>>>0)return 0;for(n=u+1|0,15<=a>>>0?(df[0|u]=240,255<=(0|(A=a-15|0))&&(If(n,255,(c=(239+(i-(((0|A)<509?A:509)+r|0)|0)>>>0)/255|0)+1|0),A=(a+Bf(c,-255)|0)-270|0,n=2+(u+c|0)|0),df[0|n]=A,n=n+1|0):df[0|u]=a<<4,A=a+n|0;a=gf[r+4|0]|gf[r+5|0]<<8|(gf[r+6|0]<<16|gf[r+7|0]<<24),c=gf[0|r]|gf[r+1|0]<<8|(gf[r+2|0]<<16|gf[r+3|0]<<24),df[0|n]=c,df[n+1|0]=c>>>8,df[n+2|0]=c>>>16,df[n+3|0]=c>>>24,df[n+4|0]=a,df[n+5|0]=a>>>8,df[n+6|0]=a>>>16,df[n+7|0]=a>>>24,r=r+8|0,(n=n+8|0)>>>0<A>>>0;);for(r=i;;){i=r-t|0,df[0|A]=i,df[A+1|0]=i>>>8,n=t+4|0;e:{i:{if((a=s)>>>0<=(i=r+4|0)>>>0)t=i;else{if(n=(gf[0|i]|gf[i+1|0]<<8|(gf[i+2|0]<<16|gf[i+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24)))break i;n=t+8|0,t=r+8|0}if(t>>>0<a>>>0)for(;;){if(l=(gf[0|t]|gf[t+1|0]<<8|(gf[t+2|0]<<16|gf[t+3|0]<<24))^(gf[0|n]|gf[n+1|0]<<8|(gf[n+2|0]<<16|gf[n+3|0]<<24))){i=((Rf(l)>>>3|0)+t|0)-i|0;break e}if(n=n+4|0,!((t=t+4|0)>>>0<s>>>0))break}(gf[0|n]|gf[n+1|0]<<8)!=(gf[0|t]|gf[t+1|0]<<8)|m>>>0<=t>>>0||(n=n+2|0,t=t+2|0),t>>>0<w>>>0&&(t=gf[0|n]==gf[0|t]?t+1|0:t),i=t-i|0;break e}i=Rf(n)>>>3|0}if(b>>>0<8+(((i+240>>>0)/255|0)+A|0)>>>0)return 0;if(a=u,u=A+2|0,r=4+(r+i|0)|0,t=gf[0|a],15<=i>>>0?(df[0|a]=t+15,df[0|u]=255,df[u+1|0]=255,df[u+2|0]=255,df[u+3|0]=255,1020<=(t=i-15|0)>>>0&&(t=Bf(a=((i=i-1035|0)>>>0)/1020|0,-1020)+i|0,u=If(A+6|0,255,(i=a<<2)+4|0)+i|0),df[0|(a=u+(i=((65535&t)>>>0)/255|0)|0)]=i+t,u=a+1|0):df[0|a]=i+t,Z>>>0<=r>>>0)break r;if(wf[(Bf(gf[0|(i=r-2|0)]|gf[i+1|0]<<8|(gf[i+2|0]<<16|gf[i+3|0]<<24),-1640531535)>>>18&16380)+f>>2]=i-v,a=(Bf(gf[0|r]|gf[r+1|0]<<8|(gf[r+2|0]<<16|gf[r+3|0]<<24),-1640531535)>>>18&16380)+f|0,i=wf[a>>2],t=a,a=r-v|0,wf[t>>2]=a,i+65535>>>0<a>>>0||(gf[0|(t=i+v|0)]|gf[t+1|0]<<8|(gf[t+2|0]<<16|gf[t+3|0]<<24))!=(gf[0|r]|gf[r+1|0]<<8|(gf[r+2|0]<<16|gf[r+3|0]<<24)))break;A=u+1|(df[0|u]=0)}if(t=(l=r+1|0)-v|0,!((n=r+2|0)>>>0<=Z>>>0))break}if(!(b>>>0<1+(((i=k-r|0)+u|0)+((i+240>>>0)/255|0)|0)>>>0))return 15<=i>>>0?(df[0|u]=240,f=u+1|0,(a=i-15|0)>>>0<255?df[0|(u=f)]=a:(If(b=f,255,(f=((a=i-270|0)>>>0)/255|0)+1|0),df[0|(u=2+(f+u|0)|0)]=a+Bf(f,-255))):df[0|u]=i<<4,(Ef(u+1|0,r,i)+i|0)-e|0}}return J}function S(f){var r=0,e=0,i=0,a=0,b=0,k=0,o=0,t=0,n=0,s=0,u=0,c=0;_f=c=_f-16|0;f:{r:{e:{i:{a:{b:{k:{o:{t:{n:{s:{u:{if(f>>>0<=244){if(3&(r=(b=wf[509])>>>(e=(t=f>>>0<11?16:f+11&-8)>>>3|0)|0)){f=(a=wf[(r=(i=e+(1&(-1^r))|0)<<3)+2084>>2])+8|0,(0|(e=wf[a+8>>2]))!=(0|(r=r+2076|0))?(wf[e+12>>2]=r,wf[r+8>>2]=e):wf[509]=Cf(-2,i)&b,r=i<<3,wf[a+4>>2]=3|r,wf[(r=r+a|0)+4>>2]=1|wf[r+4>>2];break f}if(t>>>0<=(s=wf[511])>>>0)break u;if(r){e=f=(r=(0-(f=(0-(f=2<<e)|f)&r<<e)&f)-1|0)>>>12&16,e|=f=(r=r>>>f|0)>>>5&8,e|=f=(r=r>>>f|0)>>>2&4,k=wf[(f=(e=((e|=f=(r=r>>>f|0)>>>1&2)|(f=(r=r>>>f|0)>>>1&1))+(r>>>f|0)|0)<<3)+2084>>2],(0|(r=wf[k+8>>2]))!=(0|(f=f+2076|0))?(wf[r+12>>2]=f,wf[f+8>>2]=r):(b=Cf(-2,e)&b,wf[509]=b),f=k+8|0,wf[k+4>>2]=3|t,a=(r=e<<3)-t|0,wf[(i=k+t|0)+4>>2]=1|a,wf[r+k>>2]=a,s&&(e=2076+((r=s>>>3|0)<<3)|0,k=wf[514],r=(r=1<<r)&b?wf[e+8>>2]:(wf[509]=r|b,e),wf[e+8>>2]=k,wf[r+12>>2]=k,wf[k+12>>2]=e,wf[k+8>>2]=r),wf[514]=i,wf[511]=a;break f}if(!(o=wf[510]))break u;for(e=f=(r=(o&0-o)-1|0)>>>12&16,e|=f=(r=r>>>f|0)>>>5&8,e|=f=(r=r>>>f|0)>>>2&4,r=wf[2340+(((e|=f=(r=r>>>f|0)>>>1&2)|(f=(r=r>>>f|0)>>>1&1))+(r>>>f|0)<<2)>>2],i=(-8&wf[r+4>>2])-t|0,e=r;f=(f=wf[e+16>>2])||wf[e+20>>2];)i=(a=(e=(-8&wf[f+4>>2])-t|0)>>>0<i>>>0)?e:i,r=a?f:r,e=f;if((n=r+t|0)>>>0<=r>>>0)break s;if(u=wf[r+24>>2],(0|(a=wf[r+12>>2]))!=(0|r)){f=wf[r+8>>2],wf[f+12>>2]=a,wf[a+8>>2]=f;break r}if(!(f=wf[(e=r+20|0)>>2])){if(!(f=wf[r+16>>2]))break n;e=r+16|0}for(;k=e,(f=wf[(e=(a=f)+20|0)>>2])||(e=a+16|0,f=wf[a+16>>2]););wf[k>>2]=0;break r}if(t=-1,!(4294967231<f>>>0)&&(t=-8&(f=f+11|0),n=wf[510])){b=31,i=0-t|0,t>>>0<=16777215&&(f=f>>>8|0,f<<=k=f+1048320>>>16&8,b=28+((f=((f<<=e=f+520192>>>16&4)<<(r=f+245760>>>16&2)>>>15|0)-(r|e|k)|0)<<1|t>>>f+21&1)|0);c:{l:{if(e=wf[2340+(b<<2)>>2])for(r=t<<(31==((f=0)|b)?0:25-(b>>>1|0)|0);;){if(k=(-8&wf[e+4>>2])-t|0,!(i>>>0<=k>>>0||(a=e,i=k))){i=0,f=e;break l}if(k=wf[e+20>>2],e=wf[16+((r>>>29&4)+e|0)>>2],f=!k||(0|k)==(0|e)?f:k,r<<=1,!e)break}else f=0;if(!(f|a)){if(!(f=(0-(f=2<<b)|f)&n))break u;e=f=(r=(f&0-f)-1|0)>>>12&16,e|=f=(r=r>>>f|0)>>>5&8,e|=f=(r=r>>>f|0)>>>2&4,f=wf[2340+(((e|=f=(r=r>>>f|0)>>>1&2)|(f=(r=r>>>f|0)>>>1&1))+(r>>>f|0)<<2)>>2]}if(!f)break c}for(;i=(e=(r=(-8&wf[f+4>>2])-t|0)>>>0<i>>>0)?r:i,a=e?f:a,f=(r=wf[f+16>>2])||wf[f+20>>2];);}if(!(!a|wf[511]-t>>>0<=i>>>0)){if((o=a+t|0)>>>0<=a>>>0)break s;if(b=wf[a+24>>2],(0|a)!=(0|(r=wf[a+12>>2]))){f=wf[a+8>>2],wf[f+12>>2]=r,wf[r+8>>2]=f;break e}if(!(f=wf[(e=a+20|0)>>2])){if(!(f=wf[a+16>>2]))break t;e=a+16|0}for(;k=e,(f=wf[(e=(r=f)+20|0)>>2])||(e=r+16|0,f=wf[r+16>>2]););wf[k>>2]=0;break e}}}if(t>>>0<=(e=wf[511])>>>0){i=wf[514],16<=(r=e-t|0)>>>0?(wf[511]=r,f=i+t|0,wf[514]=f,wf[f+4>>2]=1|r,wf[e+i>>2]=r,wf[i+4>>2]=3|t):(wf[514]=0,wf[511]=0,wf[i+4>>2]=3|e,wf[(f=e+i|0)+4>>2]=1|wf[f+4>>2]),f=i+8|0;break f}if(t>>>0<(o=wf[512])>>>0){r=o-t|0,wf[512]=r,f=(e=wf[515])+t|0,wf[515]=f,wf[f+4>>2]=1|r,wf[e+4>>2]=3|t,f=e+8|0;break f}if((e=(k=(r=n=t+47|(f=0))+(e=wf[627]?wf[629]:(wf[630]=-1,wf[631]=-1,wf[628]=4096,wf[629]=4096,wf[627]=12+c&-16^1431655768,wf[632]=0,wf[620]=0,4096))|0)&(a=0-e|0))>>>0<=t>>>0)break f;if(i=wf[619],i&&i>>>0<(b=(r=wf[617])+e|0)>>>0|b>>>0<=r>>>0)break f;if(4&gf[2480])break b;u:{c:{if(i=wf[515])for(f=2484;;){if(i>>>0<(r=wf[f>>2])+wf[f+4>>2]>>>0&&r>>>0<=i>>>0)break c;if(!(f=wf[f+8>>2]))break}if(-1==(0|(r=d(0))))break k;if(b=e,(f=(i=wf[628])-1|0)&r&&(b=(e-r|0)+(f+r&0-i)|0),b>>>0<=t>>>0|2147483646<b>>>0)break k;if(i=wf[619],i&&i>>>0<(a=(f=wf[617])+b|0)>>>0|a>>>0<=f>>>0)break k;if((0|r)!=(0|(f=d(b))))break u;break a}if(2147483646<(b=a&k-o)>>>0)break k;if((0|(r=d(b)))==(wf[f>>2]+wf[f+4>>2]|0))break o;f=r}if(!(-1==(0|f)|t+48>>>0<=b>>>0)){if(2147483646<(r=(r=wf[629])+(n-b|0)&0-r)>>>0){r=f;break a}if(-1!=(0|d(r))){b=r+b|0,r=f;break a}d(0-b|0);break k}if(-1!=(0|(r=f)))break a;break k}l()}a=0;break r}r=0;break e}if(-1!=(0|r))break a}wf[620]=4|wf[620]}if(2147483646<e>>>0)break i;if(r=d(e),(f=d(0))>>>0<=r>>>0|-1==(0|r)|-1==(0|f))break i;if((b=f-r|0)>>>0<=t+40>>>0)break i}f=wf[617]+b|0,(wf[617]=f)>>>0>g[618]&&(wf[618]=f);a:{b:{k:{if(k=wf[515]){for(f=2484;;){if(((i=wf[f>>2])+(e=wf[f+4>>2])|0)==(0|r))break k;if(!(f=wf[f+8>>2]))break}break b}for((f=wf[513])>>>0<=r>>>0&&f||(wf[513]=r),f=0,wf[622]=b,wf[621]=r,wf[517]=-1,wf[518]=wf[627],wf[624]=0;e=(i=f<<3)+2076|0,wf[i+2084>>2]=e,wf[i+2088>>2]=e,32!=(0|(f=f+1|0)););e=(i=b-40|0)-(f=r+8&7?-8-r&7:0)|0,wf[512]=e,f=f+r|0,wf[515]=f,wf[f+4>>2]=1|e,wf[4+(r+i|0)>>2]=40,wf[516]=wf[631];break a}if(!(8&wf[f+12>>2]|r>>>0<=k>>>0|k>>>0<i>>>0)){wf[f+4>>2]=e+b,e=(f=k+8&7?-8-k&7:0)+k|0,wf[515]=e,f=(r=wf[512]+b|0)-f|0,wf[512]=f,wf[e+4>>2]=1|f,wf[4+(r+k|0)>>2]=40,wf[516]=wf[631];break a}}g[513]>r>>>0&&(wf[513]=r),e=r+b|0,f=2484;b:{k:{o:{t:{n:{s:{for(;(0|e)!=wf[f>>2];)if(!(f=wf[f+8>>2]))break s;if(!(8&gf[f+12|0]))break n}for(f=2484;;){if(e=wf[f>>2],e>>>0<=k>>>0&&k>>>0<(a=e+wf[f+4>>2]|0)>>>0)break t;f=wf[f+8>>2]}}if(wf[f>>2]=r,wf[f+4>>2]=wf[f+4>>2]+b,wf[(n=(r+8&7?-8-r&7:0)+r|0)+4>>2]=3|t,e=((b=e+(e+8&7?-8-e&7:0)|0)-n|0)-t|0,o=t+n|0,(0|b)==(0|k)){wf[515]=o,f=wf[512]+e|0,wf[512]=f,wf[o+4>>2]=1|f;break k}if(wf[514]==(0|b)){wf[514]=o,f=wf[511]+e|0,wf[511]=f,wf[o+4>>2]=1|f,wf[f+o>>2]=f;break k}if(1==(3&(f=wf[b+4>>2]))){k=-8&f;n:if(f>>>0<=255)i=wf[b+8>>2],f=f>>>3|0,(0|(r=wf[b+12>>2]))!=(0|i)?(wf[i+12>>2]=r,wf[r+8>>2]=i):wf[509]=wf[509]&Cf(-2,f);else{if(t=wf[b+24>>2],(0|b)==(0|(r=wf[b+12>>2])))if(f=b+20|0,i=wf[f>>2],i=i||wf[(f=b+16|0)>>2]){for(;a=f,(i=wf[(f=(r=i)+20|0)>>2])||(f=r+16|0,i=wf[r+16>>2]););wf[a>>2]=0}else r=0;else f=wf[b+8>>2],wf[f+12>>2]=r,wf[r+8>>2]=f;if(t){i=wf[b+28>>2];s:{if(wf[(f=2340+(i<<2)|0)>>2]==(0|b)){if(wf[f>>2]=r)break s;wf[510]=wf[510]&Cf(-2,i);break n}if(!(wf[t+(wf[t+16>>2]==(0|b)?16:20)>>2]=r))break n}wf[r+24>>2]=t,(f=wf[b+16>>2])&&(wf[r+16>>2]=f,wf[f+24>>2]=r),(f=wf[b+20>>2])&&(wf[r+20>>2]=f,wf[f+24>>2]=r)}}b=b+k|0,e=e+k|0}if(wf[b+4>>2]=-2&wf[b+4>>2],wf[o+4>>2]=1|e,(wf[e+o>>2]=e)>>>0<=255){r=2076+((f=e>>>3|0)<<3)|0,f=(e=wf[509])&(f=1<<f)?wf[r+8>>2]:(wf[509]=f|e,r),wf[r+8>>2]=o,wf[f+12>>2]=o,wf[o+12>>2]=r,wf[o+8>>2]=f;break k}if(f=31,e>>>0<=16777215&&(f=e>>>8|0,f<<=a=f+1048320>>>16&8,f=28+((f=((f<<=i=f+520192>>>16&4)<<(r=f+245760>>>16&2)>>>15|0)-(r|i|a)|0)<<1|e>>>f+21&1)|0),wf[o+28>>2]=f,wf[o+16>>2]=0,a=2340+(f<<2)|(wf[o+20>>2]=0),(i=wf[510])&(r=1<<f)){for(f=e<<(31==(0|f)?0:25-(f>>>1|0)|0),r=wf[a>>2];;){if((-8&wf[(i=r)+4>>2])==(0|e))break o;if(r=f>>>29|0,f<<=1,!(r=wf[(a=i+(4&r)|0)+16>>2]))break}wf[a+16>>2]=o,wf[o+24>>2]=i}else wf[510]=r|i,wf[a>>2]=o,wf[o+24>>2]=a;wf[o+12>>2]=o,wf[o+8>>2]=o;break k}for(e=(i=b-40|0)-(f=r+8&7?-8-r&7:0)|0,wf[512]=e,f=f+r|0,wf[515]=f,wf[f+4>>2]=1|e,wf[4+(r+i|0)>>2]=40,wf[516]=wf[631],wf[(e=(f=(a+(a-39&7?39-a&7:0)|0)-47|0)>>>0<k+16>>>0?k:f)+4>>2]=27,f=wf[624],wf[e+16>>2]=wf[623],wf[e+20>>2]=f,f=wf[622],wf[e+8>>2]=wf[621],wf[e+12>>2]=f,wf[623]=e+8,wf[622]=b,wf[621]=r,f=e+24|(wf[624]=0);wf[f+4>>2]=7,r=f+8|0,f=f+4|0,r>>>0<a>>>0;);if((0|e)==(0|k))break a;if(wf[e+4>>2]=-2&wf[e+4>>2],a=e-k|0,wf[k+4>>2]=1|a,(wf[e>>2]=a)>>>0<=255){r=2076+((f=a>>>3|0)<<3)|0,f=(e=wf[509])&(f=1<<f)?wf[r+8>>2]:(wf[509]=f|e,r),wf[r+8>>2]=k,wf[f+12>>2]=k,wf[k+12>>2]=r,wf[k+8>>2]=f;break a}if(f=31,wf[k+16>>2]=0,a>>>(wf[k+20>>2]=0)<=16777215&&(f=a>>>8|0,f<<=i=f+1048320>>>16&8,f=28+((f=((f<<=e=f+520192>>>16&4)<<(r=f+245760>>>16&2)>>>15|0)-(r|e|i)|0)<<1|a>>>f+21&1)|0),i=2340+((wf[k+28>>2]=f)<<2)|0,(e=wf[510])&(r=1<<f)){for(f=a<<(31==(0|f)?0:25-(f>>>1|0)|0),r=wf[i>>2];;){if((0|a)==(-8&wf[(e=r)+4>>2]))break b;if(r=f>>>29|0,f<<=1,!(r=wf[(i=e+(4&r)|0)+16>>2]))break}wf[i+16>>2]=k,wf[k+24>>2]=e}else wf[510]=r|e,wf[i>>2]=k,wf[k+24>>2]=i;wf[k+12>>2]=k,wf[k+8>>2]=k;break a}f=wf[i+8>>2],wf[f+12>>2]=o,wf[i+8>>2]=o,wf[o+24>>2]=0,wf[o+12>>2]=i,wf[o+8>>2]=f}f=n+8|0;break f}f=wf[e+8>>2],wf[f+12>>2]=k,wf[e+8>>2]=k,wf[k+24>>2]=0,wf[k+12>>2]=e,wf[k+8>>2]=f}if(!((f=wf[512])>>>0<=t>>>0)){r=f-t|0,wf[512]=r,f=(e=wf[515])+t|0,wf[515]=f,wf[f+4>>2]=1|r,wf[e+4>>2]=3|t,f=e+8|0;break f}}wf[508]=48,f=0;break f}e:if(b){e=wf[a+28>>2];i:{if(wf[(f=2340+(e<<2)|0)>>2]==(0|a)){if(wf[f>>2]=r)break i;n=Cf(-2,e)&n,wf[510]=n;break e}if(!(wf[b+(wf[b+16>>2]==(0|a)?16:20)>>2]=r))break e}wf[r+24>>2]=b,(f=wf[a+16>>2])&&(wf[r+16>>2]=f,wf[f+24>>2]=r),(f=wf[a+20>>2])&&(wf[r+20>>2]=f,wf[f+24>>2]=r)}e:if(i>>>0<=15)f=i+t|0,wf[a+4>>2]=3|f,wf[(f=f+a|0)+4>>2]=1|wf[f+4>>2];else if(wf[a+4>>2]=3|t,wf[o+4>>2]=1|i,(wf[i+o>>2]=i)>>>0<=255)r=2076+((f=i>>>3|0)<<3)|0,f=(e=wf[509])&(f=1<<f)?wf[r+8>>2]:(wf[509]=f|e,r),wf[r+8>>2]=o,wf[f+12>>2]=o,wf[o+12>>2]=r,wf[o+8>>2]=f;else{f=31,i>>>0<=16777215&&(f=i>>>8|0,f<<=k=f+1048320>>>16&8,f=28+((f=((f<<=e=f+520192>>>16&4)<<(r=f+245760>>>16&2)>>>15|0)-(r|e|k)|0)<<1|i>>>f+21&1)|0),wf[o+28>>2]=f,wf[o+16>>2]=0,e=2340+(f<<2)|(wf[o+20>>2]=0);i:{if((r=1<<f)&n){for(f=i<<(31==(0|f)?0:25-(f>>>1|0)|0),t=wf[e>>2];;){if((-8&wf[(r=t)+4>>2])==(0|i))break i;if(e=f>>>29|0,f<<=1,!(t=wf[(e=r+(4&e)|0)+16>>2]))break}wf[e+16>>2]=o,wf[o+24>>2]=r}else wf[510]=r|n,wf[e>>2]=o,wf[o+24>>2]=e;wf[o+12>>2]=o,wf[o+8>>2]=o;break e}f=wf[r+8>>2],wf[f+12>>2]=o,wf[r+8>>2]=o,wf[o+24>>2]=0,wf[o+12>>2]=r,wf[o+8>>2]=f}f=a+8|0;break f}r:if(u){e=wf[r+28>>2];e:{if(wf[(f=2340+(e<<2)|0)>>2]==(0|r)){if(wf[f>>2]=a)break e;wf[510]=Cf(-2,e)&o;break r}if(!(wf[(wf[u+16>>2]==(0|r)?16:20)+u>>2]=a))break r}wf[a+24>>2]=u,(f=wf[r+16>>2])&&(wf[a+16>>2]=f,wf[f+24>>2]=a),(f=wf[r+20>>2])&&(wf[a+20>>2]=f,wf[f+24>>2]=a)}i>>>0<=15?(f=i+t|0,wf[r+4>>2]=3|f,wf[(f=f+r|0)+4>>2]=1|wf[f+4>>2]):(wf[r+4>>2]=3|t,wf[n+4>>2]=1|i,wf[i+n>>2]=i,s&&(e=2076+((f=s>>>3|0)<<3)|0,a=wf[514],f=(f=1<<f)&b?wf[e+8>>2]:(wf[509]=f|b,e),wf[e+8>>2]=a,wf[f+12>>2]=a,wf[a+12>>2]=e,wf[a+8>>2]=f),wf[514]=n,wf[511]=i),f=r+8|0}return _f=16+c|0,f}function B(f){var r,e=0,i=0,a=0,b=0,k=0,o=0;f:if(f){r=(a=f-8|0)+(f=-8&(e=wf[f-4>>2]))|0;r:if(!(1&e)){if(!(3&e))break f;if((a=a-(e=wf[a>>2])|0)>>>0<g[513])break f;if(f=f+e|0,wf[514]==(0|a)){if(3==(3&(e=wf[4+r>>2])))return wf[511]=f,wf[4+r>>2]=-2&e,wf[a+4>>2]=1|f,void(wf[f+a>>2]=f)}else if(e>>>0<=255)b=wf[a+8>>2],e=e>>>3|0,(0|(i=wf[a+12>>2]))!=(0|b)?(wf[b+12>>2]=i,wf[i+8>>2]=b):wf[509]=wf[509]&Cf(-2,e);else{if(o=wf[a+24>>2],(0|a)==(0|(e=wf[a+12>>2])))if(b=a+20|0,i=wf[b>>2],i=i||wf[(b=a+16|0)>>2]){for(;k=b,(i=wf[(b=(e=i)+20|0)>>2])||(b=e+16|0,i=wf[e+16>>2]););wf[k>>2]=0}else e=0;else i=wf[a+8>>2],wf[i+12>>2]=e,wf[e+8>>2]=i;if(o){b=wf[a+28>>2];e:{if(wf[(i=2340+(b<<2)|0)>>2]==(0|a)){if(wf[i>>2]=e)break e;wf[510]=wf[510]&Cf(-2,b);break r}if(!(wf[o+(wf[o+16>>2]==(0|a)?16:20)>>2]=e))break r}wf[e+24>>2]=o,(i=wf[a+16>>2])&&(wf[e+16>>2]=i,wf[i+24>>2]=e),(i=wf[a+20>>2])&&(wf[e+20>>2]=i,wf[i+24>>2]=e)}}}if(!(r>>>0<=a>>>0)&&1&(e=wf[4+r>>2])){r:{if(!(2&e)){if(wf[515]==(0|r)){if(wf[515]=a,f=wf[512]+f|0,wf[512]=f,wf[a+4>>2]=1|f,wf[514]!=(0|a))break f;return wf[511]=0,void(wf[514]=0)}if(wf[514]==(0|r))return wf[514]=a,f=wf[511]+f|0,wf[511]=f,wf[a+4>>2]=1|f,void(wf[f+a>>2]=f);f=(-8&e)+f|0;e:if(e>>>0<=255)b=wf[8+r>>2],e=e>>>3|0,(0|(i=wf[12+r>>2]))!=(0|b)?(wf[b+12>>2]=i,wf[i+8>>2]=b):wf[509]=wf[509]&Cf(-2,e);else{if(o=wf[24+r>>2],(0|r)==(0|(e=wf[12+r>>2])))if(b=20+r|0,i=wf[b>>2],i=i||wf[(b=16+r|0)>>2]){for(;k=b,(i=wf[(b=(e=i)+20|0)>>2])||(b=e+16|0,i=wf[e+16>>2]););wf[k>>2]=0}else e=0;else i=wf[8+r>>2],wf[i+12>>2]=e,wf[e+8>>2]=i;if(o){b=wf[28+r>>2];i:{if(wf[(i=2340+(b<<2)|0)>>2]==(0|r)){if(wf[i>>2]=e)break i;wf[510]=wf[510]&Cf(-2,b);break e}if(!(wf[o+(wf[o+16>>2]==(0|r)?16:20)>>2]=e))break e}wf[e+24>>2]=o,(i=wf[16+r>>2])&&(wf[e+16>>2]=i,wf[i+24>>2]=e),(i=wf[20+r>>2])&&(wf[e+20>>2]=i,wf[i+24>>2]=e)}}if(wf[a+4>>2]=1|f,wf[f+a>>2]=f,wf[514]!=(0|a))break r;return void(wf[511]=f)}wf[4+r>>2]=-2&e,wf[a+4>>2]=1|f,wf[f+a>>2]=f}if(f>>>0<=255)return e=2076+((f=f>>>3|0)<<3)|0,f=(i=wf[509])&(f=1<<f)?wf[e+8>>2]:(wf[509]=f|i,e),wf[e+8>>2]=a,wf[f+12>>2]=a,wf[a+12>>2]=e,void(wf[a+8>>2]=f);b=31,wf[a+16>>2]=0,f>>>(wf[a+20>>2]=0)<=16777215&&(e=f>>>8|0,e<<=k=e+1048320>>>16&8,b=28+((e=((e<<=b=e+520192>>>16&4)<<(i=e+245760>>>16&2)>>>15|0)-(i|b|k)|0)<<1|f>>>e+21&1)|0),k=2340+((wf[a+28>>2]=b)<<2)|0;r:{e:{if((i=wf[510])&(e=1<<b)){for(b=f<<(31==(0|b)?0:25-(b>>>1|0)|0),e=wf[k>>2];;){if((-8&wf[(i=e)+4>>2])==(0|f))break e;if(e=b>>>29|0,b<<=1,!(e=wf[(k=i+(4&e)|0)+16>>2]))break}wf[k+16>>2]=a,wf[a+24>>2]=i}else wf[510]=e|i,wf[k>>2]=a,wf[a+24>>2]=k;wf[a+12>>2]=a,wf[a+8>>2]=a;break r}f=wf[i+8>>2],wf[f+12>>2]=a,wf[i+8>>2]=a,wf[a+24>>2]=0,wf[a+12>>2]=i,wf[a+8>>2]=f}f=wf[517]-1|0,wf[517]=f||-1}}}function b(f,r,e,i){var a=0,b=0,k=0,o=0;_f=o=_f+-64|0,b=-11;f:if(!(e>>>0<19)){wf[(e=o)+56>>2]=0,wf[e+60>>2]=0,wf[e+48>>2]=0,wf[e+52>>2]=0,wf[e+40>>2]=0,wf[e+44>>2]=0,wf[e+32>>2]=0,wf[e+36>>2]=0,wf[e+24>>2]=0,wf[e+28>>2]=0,wf[e+16>>2]=0,wf[e+20>>2]=0,wf[e+8>>2]=0,wf[e+12>>2]=0,e=wf[(a=k=i||e+8|0)+52>>2],wf[f+48>>2]=wf[a+48>>2],wf[f+52>>2]=e,e=wf[a+44>>2],wf[f+40>>2]=wf[a+40>>2],wf[f+44>>2]=e,e=wf[a+36>>2],i=wf[a+32>>2],wf[f+32>>2]=i,wf[f+36>>2]=e,e=wf[a+28>>2],wf[f+24>>2]=wf[a+24>>2],wf[f+28>>2]=e,e=wf[a+20>>2],wf[f+16>>2]=wf[a+16>>2],wf[f+20>>2]=e,e=wf[a+12>>2],wf[f+8>>2]=wf[a+8>>2],wf[f+12>>2]=e,e=wf[a+4>>2],wf[f>>2]=wf[a>>2],wf[f+4>>2]=e;r:{if((a=(0|i)<3?1:2)>>>0>Sf[f+148>>1]){if(B(wf[f+144>>2]),i=(wf[(e=f)+32>>2]<=2?function(){var f=0;if(!(f=S(16416)))return 0;7&f||If(f,0,16416);return f}:function(){var f=0;if(!(f=S(262200)))return 0;3&f||(wf[f+262144>>2]=-1,wf[f+262148>>2]=0,wf[f+262168>>2]=9,wf[f+262172>>2]=0);return f})(),b=-9,!(wf[e+144>>2]=i))break f;vf[f+148>>1]=a,e=f+150|0}else{if(Sf[f+150>>1]==(0|a))break r;e=f+150|0,b=wf[f+144>>2],(0|i)<=2?!b|7&b||If(b,0,16416):(!b|3&b||(wf[b+262144>>2]=-1,wf[b+262148>>2]=0,wf[b+262168>>2]=9,wf[b+262172>>2]=0),i=(0|(i=wf[f+32>>2]))<1?9:i,vf[wf[f+144>>2]+262168>>1]=(0|i)<12?i:12)}vf[e>>1]=a}r:{if(e=wf[(i=f)>>2]){if(a=-2,4!=(-4&e))break r}else e=4,wf[f>>2]=4;a=wf[1200+(e<<2)>>2]}if(wf[i+68>>2]=a,e=!wf[f+4>>2],(e=wf[k+36>>2]?e<<16:a+(e<<17)|0)>>>0<=g[f+72>>2])i=wf[f+76>>2];else{if(wf[f+72>>2]=0,B(wf[f+76>>2]),i=J(e),b=-9,!(wf[f+76>>2]=i))break f;wf[f+72>>2]=e}wf[f+84>>2]=0,wf[f+80>>2]=i,R(f+96|0),wf[f+64>>2]=0,wf[f+4>>2]||(i=wf[f+144>>2],(0|(e=wf[f+32>>2]))<=2?(p(i),s(i,0)):(m(i,e),wf[i+262172>>2]=0)),3<=wf[k+32>>2]&&(df[wf[f+144>>2]+262170|0]=0!=wf[k+40>>2]),df[0|r]=4,df[r+1|0]=34,df[r+2|0]=77,df[r+3|0]=24,df[r+4|0]=wf[f+8>>2]<<2&4|(wf[f+28>>2]<<4&16|wf[f+4>>2]<<5&32|(0!=wf[f+16>>2]|0!=wf[f+20>>2])<<3)|0!=wf[f+24>>2]|64,df[r+5|0]=gf[0|f]<<4&112,a=r+6|0,(i=k=wf[f+20>>2])|(e=wf[f+16>>2])&&(df[r+6|0]=e,df[r+7|0]=e>>>8,df[r+8|0]=e>>>16,df[r+9|0]=e>>>24,df[r+10|0]=i,df[r+11|0]=i>>>8,df[r+12|0]=i>>>16,df[r+13|0]=i>>>24,wf[f+88>>2]=0,a=r+14|(wf[f+92>>2]=0)),e=a,(i=wf[f+24>>2])&&(df[0|e]=i,df[e+1|0]=i>>>8,df[e+2|0]=i>>>16,df[e+3|0]=i>>>24,e=e+4|0),i=r+4|0,df[0|e]=Z(i,e-i|0)>>>8,b=(wf[f+60>>2]=1)+(e-r|0)|0}return _f=o+64|0,b}function t(f,r,e){f:{r:switch((15&e)-1|0){case 11:f=Bf(Cf(Bf(gf[0|r]|gf[r+1|0]<<8|(gf[r+2|0]<<16|gf[r+3|0]<<24),-1028477379)+f|0,17),668265263),r=r+4|0;case 7:f=Bf(Cf(Bf(gf[0|r]|gf[r+1|0]<<8|(gf[r+2|0]<<16|gf[r+3|0]<<24),-1028477379)+f|0,17),668265263),r=r+4|0;case 3:f=Bf(Cf(Bf(gf[0|r]|gf[r+1|0]<<8|(gf[r+2|0]<<16|gf[r+3|0]<<24),-1028477379)+f|0,17),668265263);break f;case 12:f=Bf(Cf(Bf(gf[0|r]|gf[r+1|0]<<8|(gf[r+2|0]<<16|gf[r+3|0]<<24),-1028477379)+f|0,17),668265263),r=r+4|0;case 8:f=Bf(Cf(Bf(gf[0|r]|gf[r+1|0]<<8|(gf[r+2|0]<<16|gf[r+3|0]<<24),-1028477379)+f|0,17),668265263),r=r+4|0;case 4:f=Bf(Cf(Bf(Cf(Bf(gf[0|r]|gf[r+1|0]<<8|(gf[r+2|0]<<16|gf[r+3|0]<<24),-1028477379)+f|0,17),668265263)+Bf(gf[r+4|0],374761393)|0,11),-1640531535);break f;case 13:f=Bf(Cf(Bf(gf[0|r]|gf[r+1|0]<<8|(gf[r+2|0]<<16|gf[r+3|0]<<24),-1028477379)+f|0,17),668265263),r=r+4|0;case 9:f=Bf(Cf(Bf(gf[0|r]|gf[r+1|0]<<8|(gf[r+2|0]<<16|gf[r+3|0]<<24),-1028477379)+f|0,17),668265263),r=r+4|0;case 5:f=Bf(Cf(Bf(Cf(Bf(Cf(Bf(gf[0|r]|gf[r+1|0]<<8|(gf[r+2|0]<<16|gf[r+3|0]<<24),-1028477379)+f|0,17),668265263)+Bf(gf[r+4|0],374761393)|0,11),-1640531535)+Bf(gf[r+5|0],374761393)|0,11),-1640531535);break f;case 14:f=Bf(Cf(Bf(gf[0|r]|gf[r+1|0]<<8|(gf[r+2|0]<<16|gf[r+3|0]<<24),-1028477379)+f|0,17),668265263),r=r+4|0;case 10:f=Bf(Cf(Bf(gf[0|r]|gf[r+1|0]<<8|(gf[r+2|0]<<16|gf[r+3|0]<<24),-1028477379)+f|0,17),668265263),r=r+4|0;case 6:f=Bf(Cf(Bf(gf[0|r]|gf[r+1|0]<<8|(gf[r+2|0]<<16|gf[r+3|0]<<24),-1028477379)+f|0,17),668265263),r=r+4|0;case 2:f=Bf(Cf(Bf(gf[0|r],374761393)+f|0,11),-1640531535),r=r+1|0;case 1:f=Bf(Cf(Bf(gf[0|r],374761393)+f|0,11),-1640531535),r=r+1|0;break;case 0:break r;default:break f}f=Bf(Cf(Bf(gf[0|r],374761393)+f|0,11),-1640531535)}return f=Bf(f>>>15^f,-2048144777),(f=Bf(f>>>13^f,-1028477379))>>>16^f}function _(f,r,e){var i,a,b=0,k=0,o=0;if(r){if(b=wf[f>>2]+e|0,wf[f>>2]=b,wf[f+4>>2]=wf[f+4>>2]|15<(e|b)>>>0,(b=wf[f+40>>2])+e>>>0<=15)return Ef(24+(f+b|0)|0,r,e),void(wf[f+40>>2]=wf[f+40>>2]+e);if(i=r+e|0,b&&(Ef(b+(f+24|0)|0,r,16-b|0),e=wf[f+40>>2],wf[f+40>>2]=0,wf[f+8>>2]=Bf(Cf(wf[f+8>>2]+Bf(gf[f+24|0]|gf[f+25|0]<<8|(gf[f+26|0]<<16|gf[f+27|0]<<24),-2048144777)|0,13),-1640531535),wf[f+12>>2]=Bf(Cf(wf[f+12>>2]+Bf(gf[f+28|0]|gf[f+29|0]<<8|(gf[f+30|0]<<16|gf[f+31|0]<<24),-2048144777)|0,13),-1640531535),wf[f+16>>2]=Bf(Cf(wf[f+16>>2]+Bf(gf[f+32|0]|gf[f+33|0]<<8|(gf[f+34|0]<<16|gf[f+35|0]<<24),-2048144777)|0,13),-1640531535),wf[f+20>>2]=Bf(Cf(wf[f+20>>2]+Bf(gf[f+36|0]|gf[f+37|0]<<8|(gf[f+38|0]<<16|gf[f+39|0]<<24),-2048144777)|0,13),-1640531535),r=16+(r-e|0)|0),r>>>0<=(a=i-16|0)>>>0){for(e=wf[f+20>>2],b=wf[f+16>>2],k=wf[f+12>>2],o=wf[f+8>>2];e=Bf(Cf(Bf(gf[r+12|0]|gf[r+13|0]<<8|(gf[r+14|0]<<16|gf[r+15|0]<<24),-2048144777)+e|0,13),-1640531535),b=Bf(Cf(Bf(gf[r+8|0]|gf[r+9|0]<<8|(gf[r+10|0]<<16|gf[r+11|0]<<24),-2048144777)+b|0,13),-1640531535),k=Bf(Cf(Bf(gf[r+4|0]|gf[r+5|0]<<8|(gf[r+6|0]<<16|gf[r+7|0]<<24),-2048144777)+k|0,13),-1640531535),o=Bf(Cf(Bf(gf[0|r]|gf[r+1|0]<<8|(gf[r+2|0]<<16|gf[r+3|0]<<24),-2048144777)+o|0,13),-1640531535),(r=r+16|0)>>>0<=a>>>0;);wf[f+20>>2]=e,wf[f+16>>2]=b,wf[f+12>>2]=k,wf[f+8>>2]=o}r>>>0<i>>>0&&(Ef(f+24|0,e=r,r=i-r|0),wf[f+40>>2]=r)}}function Z(f,r){var e=0,i=0,a=0,b=0,k=0;if(!(3&f)){if(16<=r>>>0){for(e=1640531535,i=-2048144777,a=606290984,k=(f+r|0)-15|0;e=Bf(Cf(Bf(gf[f+12|0]|gf[f+13|0]<<8|(gf[f+14|0]<<16|gf[f+15|0]<<24),-2048144777)+e|0,13),-1640531535),b=Bf(Cf(Bf(gf[f+8|0]|gf[f+9|0]<<8|(gf[f+10|0]<<16|gf[f+11|0]<<24),-2048144777)+b|0,13),-1640531535),i=Bf(Cf(Bf(gf[f+4|0]|gf[f+5|0]<<8|(gf[f+6|0]<<16|gf[f+7|0]<<24),-2048144777)+i|0,13),-1640531535),a=Bf(Cf(Bf(gf[0|f]|gf[f+1|0]<<8|(gf[f+2|0]<<16|gf[f+3|0]<<24),-2048144777)+a|0,13),-1640531535),(f=f+16|0)>>>0<k>>>0;);e=((Cf(i,7)+Cf(a,1)|0)+Cf(b,12)|0)+Cf(e,18)|0}else e=374761393;return t(e+r|0,f,15&r)}if(16<=r>>>0){for(e=1640531535,i=-2048144777,a=606290984,k=(f+r|0)-15|0;e=Bf(Cf(Bf(gf[f+12|0]|gf[f+13|0]<<8|(gf[f+14|0]<<16|gf[f+15|0]<<24),-2048144777)+e|0,13),-1640531535),b=Bf(Cf(Bf(gf[f+8|0]|gf[f+9|0]<<8|(gf[f+10|0]<<16|gf[f+11|0]<<24),-2048144777)+b|0,13),-1640531535),i=Bf(Cf(Bf(gf[f+4|0]|gf[f+5|0]<<8|(gf[f+6|0]<<16|gf[f+7|0]<<24),-2048144777)+i|0,13),-1640531535),a=Bf(Cf(Bf(gf[0|f]|gf[f+1|0]<<8|(gf[f+2|0]<<16|gf[f+3|0]<<24),-2048144777)+a|0,13),-1640531535),(f=f+16|0)>>>0<k>>>0;);e=((Cf(i,7)+Cf(a,1)|0)+Cf(b,12)|0)+Cf(e,18)|0}else e=374761393;return t(e+r|0,f,15&r)}function E(f,r,e){var i,a,b,k,o,t,n=0,s=0,n=-12;if(!(e>>>0<7)){if(wf[f>>2]=0,wf[f+4>>2]=0,wf[f+24>>2]=0,wf[f+28>>2]=0,wf[f+16>>2]=0,wf[f+20>>2]=0,wf[f+8>>2]=0,wf[f+12>>2]=0,407710288==(-16&(i=gf[0|r]|gf[r+1|0]<<8|(gf[r+2|0]<<16|gf[r+3|0]<<24))))return wf[f+12>>2]=1,(f+188|0)==(0|r)?(wf[f+64>>2]=8,wf[f+60>>2]=e,wf[f+36>>2]=13,e):(wf[f+36>>2]=12,4);if(n=-13,407708164==(0|i)){if(wf[f+12>>2]=0,2&(s=gf[r+4|0]))return-8;if(n=-6,64==(192&s)){if(e>>>0<(i=(7|(k=8&s))+((o=1&s)<<2)|0)>>>0)return(0|(s=f+188|0))!=(0|r)&&Ef(s,r,e),wf[f+64>>2]=i,wf[f+60>>2]=e,wf[f+36>>2]=1,e;if(128&(a=gf[r+5|0]))return-8;n=-2,(b=a>>>4&7)>>>0<4||(n=-8,15&a||(e=Z(r+4|0,i-5|0),n=-17,gf[(t=r+i|0)-1|0]==(e>>>8&255)&&(wf[f+28>>2]=s>>>4&1,wf[f+4>>2]=s>>>5&1,wf[f+8>>2]=s>>>2&1,wf[f>>2]=b,wf[f+48>>2]=64&a?wf[1200+(b<<2)>>2]:-2,k&&(e=gf[r+10|0]|gf[r+11|0]<<8|(gf[r+12|0]<<16|gf[r+13|0]<<24),r=gf[r+6|0]|gf[r+7|0]<<8|(gf[r+8|0]<<16|gf[r+9|0]<<24),wf[f+16>>2]=r,wf[f+20>>2]=e,wf[f+40>>2]=r,wf[f+44>>2]=e),o&&(r=t-5|0,wf[f+24>>2]=gf[0|r]|gf[r+1|0]<<8|(gf[r+2|0]<<16|gf[r+3|0]<<24)),wf[f+36>>2]=2,n=i)))}}}return n}function Ef(f,r,e){var i,a,b=0;if(512<=e>>>0)return k(0|f,0|r,0|e),f;if(i=f+e|0,3&(f^r))if(i>>>0<4)e=f;else if((b=i-4|0)>>>0<f>>>0)e=f;else for(e=f;df[0|e]=gf[0|r],df[e+1|0]=gf[r+1|0],df[e+2|0]=gf[r+2|0],df[e+3|0]=gf[r+3|0],r=r+4|0,(e=e+4|0)>>>0<=b>>>0;);else{f:if((0|e)<1)e=f;else if(3&f)for(e=f;;){if(df[0|e]=gf[0|r],r=r+1|0,i>>>0<=(e=e+1|0)>>>0)break f;if(!(3&e))break}else e=f;if(b=-4&i,!(b>>>0<64||(a=b+-64|0)>>>0<e>>>0))for(;wf[e>>2]=wf[r>>2],wf[e+4>>2]=wf[r+4>>2],wf[e+8>>2]=wf[r+8>>2],wf[e+12>>2]=wf[r+12>>2],wf[e+16>>2]=wf[r+16>>2],wf[e+20>>2]=wf[r+20>>2],wf[e+24>>2]=wf[r+24>>2],wf[e+28>>2]=wf[r+28>>2],wf[e+32>>2]=wf[r+32>>2],wf[e+36>>2]=wf[r+36>>2],wf[e+40>>2]=wf[r+40>>2],wf[e+44>>2]=wf[r+44>>2],wf[e+48>>2]=wf[r+48>>2],wf[e+52>>2]=wf[r+52>>2],wf[e+56>>2]=wf[r+56>>2],wf[e+60>>2]=wf[r+60>>2],r=r- -64|0,(e=e- -64|0)>>>0<=a>>>0;);if(!(b>>>0<=e>>>0))for(;wf[e>>2]=wf[r>>2],r=r+4|0,(e=e+4|0)>>>0<b>>>0;);}if(e>>>0<i>>>0)for(;df[0|e]=gf[0|r],r=r+1|0,(0|i)!=(0|(e=e+1|0)););return f}function I(f,r,e){var i=0;f:if((0|f)!=(0|r)){if((r-f|0)-e>>>0<=0-(e<<1)>>>0)return Ef(f,r,e);if(i=3&(f^r),f>>>0<r>>>0){if(i)i=f;else{if(3&f)for(i=f;;){if(!e)break f;if(df[0|i]=gf[0|r],r=r+1|0,e=e-1|0,!(3&(i=i+1|0)))break}else i=f;if(!(e>>>0<=3))for(;wf[i>>2]=wf[r>>2],r=r+4|0,i=i+4|0,3<(e=e-4|0)>>>0;);}if(e)for(;df[0|i]=gf[0|r],i=i+1|0,r=r+1|0,e=e-1|0;);}else{if(!i){if(f+e&3)for(;;){if(!e)break f;if(df[0|(i=(e=e-1|0)+f|0)]=gf[r+e|0],!(3&i))break}if(!(e>>>0<=3))for(;wf[(e=e-4|0)+f>>2]=wf[r+e>>2],3<e>>>0;);}if(e)for(;df[(e=e-1|0)+f|0]=gf[r+e|0],e;);}}return f}function If(f,r,e){var i,a,b,k,o,t,n,s=0;if(e&&(a=f+e|0,df[a-1|0]=r,df[0|f]=r,!(e>>>0<3||(df[a-2|0]=r,df[f+1|0]=r,df[a-3|0]=r,df[f+2|0]=r,e>>>0<7||(df[a-4|0]=r,df[f+3|0]=r,e>>>0<9||(a=0-f&3,i=a+f|0,s=Bf(255&r,16843009),wf[i>>2]=s,r=e-a&-4,e=r+i|0,wf[e-4>>2]=s,r>>>0<9||(wf[8+i>>2]=s,wf[4+i>>2]=s,wf[e-8>>2]=s,wf[e-12>>2]=s,r>>>0<25||(wf[24+i>>2]=s,wf[20+i>>2]=s,wf[16+i>>2]=s,wf[12+i>>2]=s,wf[e-16>>2]=s,wf[e-20>>2]=s,wf[e-24>>2]=s,wf[e-28>>2]=s,(e=(e=r)-(r=4&i|24)|0)>>>0<32))))))))for(n=Bf(k=(b=s)>>>16|(n=t=o=k=0),0),u=b+n+((t=((o=65535&b)>>>16|0)+k|0)>>>16)+((k=65535&t)>>>16)|0,s=65535&o|k<<16,a=u,r=r+i|0;wf[r+24>>2]=s,i=a,wf[r+28>>2]=i,wf[r+16>>2]=s,wf[r+20>>2]=i,wf[r+8>>2]=s,wf[r+12>>2]=i,wf[r>>2]=s,wf[r+4>>2]=i,r=r+32|0,31<(e=e-32|0)>>>0;);return f}function s(f,r){var e=0,i=0,a=f;f:{r:{if(Sf[f+16388>>1]){if(If(f,0,16404),i=0,!r)break f}else{e:{i:{a:switch(Sf[f+16390>>1]){case 2:if((e=wf[f+16384>>2])>>>0<1073741825)break i;default:e=0,If(f,vf[f+16390>>1]=0,16388);break e;case 0:break a}e=wf[f+16384>>2]}e?(e=e+65536|0,wf[f+16384>>2]=e):e=0}if(wf[f+16400>>2]=0,i=wf[f+16392>>2]=0,!r)break f;if(e)break r}wf[f+16384>>2]=65536}i=wf[r+16400>>2]?r:0}wf[a+16396>>2]=i}function c(f,r,e){var i,a,b,k,o,t,n,s,u=0,c=0,l=0;if(a=r,b=e,s=n=0,4294967276<(u=(n=wf[(i=f)+84>>2])?(s=-1,1==wf[i+60>>2]&&(s=-11,b>>>0<n+8>>>0||(k=wf[i+28>>2],o=wf[i+80>>2],s=a+4|0,b=wf[i+32>>2],t=1==wf[i+4>>2],(b=0|F[0|((0|b)<3?t?1:2:t?3:4)](wf[i+144>>2],o,s,n,n-1|0,b,wf[i+64>>2]))?(df[0|a]=b,df[a+1|0]=b>>>8,df[a+2|0]=b>>>16,df[a+3|0]=b>>>24,n=b):(df[0|a]=n,df[a+2|0]=n>>>16,df[a+1|0]=n>>>8,df[a+3|0]=n>>>24|128,Ef(s,o,n)),k&&(a=n+s|0,b=Z(s,n),df[0|a]=b,df[a+1|0]=b>>>8,df[a+2|0]=b>>>16,df[a+3|0]=b>>>24),b=(k<<2)+n|0,wf[i+4>>2]?a=wf[i+80>>2]:(a=wf[i+80>>2]+wf[i+84>>2]|0,wf[i+80>>2]=a),s=b+4|0,wf[i+84>>2]=0,b=wf[i+76>>2],wf[i+68>>2]+a>>>0<=b+wf[i+72>>2]>>>0||(b=(wf[(a=i)+32>>2]<=2?w:y)(wf[i+144>>2],b),wf[a+80>>2]=b+wf[i+76>>2]))),s):0)>>>0)return u;c=-11;f:if(!((l=e-u|0)>>>0<4)){if(df[0|(e=r+u|0)]=0,df[e+1|0]=0,df[e+2|0]=0,e=e+4|(df[e+3|0]=0),1==wf[f+8>>2]){if(u=C(f+96|0),l>>>0<8)break f;df[0|e]=u,df[e+1|0]=u>>>8,df[e+2|0]=u>>>16,df[e+3|0]=u>>>24,e=e+4|0}wf[f+72>>2]=0,wf[f+60>>2]=0,(u=c=wf[f+20>>2])|(l=wf[f+16>>2])&&(c=-14,(0|l)!=wf[f+88>>2]|wf[f+92>>2]!=(0|u))||(c=e-r|0)}return c}function h(f,r,e,i,a,b,k,o,t,n,s){var u,c,l=S(64);return u=l+4|0,c=0,4294967276<((c=J(152))?(wf[c+60>>2]=0,wf[c+56>>2]=100,wf[u>>2]=c,0):-9)>>>0?(B(l),0):(wf[l+8>>2]=f,wf[l+60>>2]=0,wf[l+52>>2]=0,wf[l+56>>2]=0,wf[l+48>>2]=s,wf[l+44>>2]=n,wf[l+40>>2]=t,wf[l+36>>2]=o,wf[l+32>>2]=k,wf[l+24>>2]=a,wf[l+28>>2]=b,wf[l+20>>2]=i,wf[l+16>>2]=e,wf[l+12>>2]=r,k=l+8|0,wf[24+(a=_f-32|(i=b=a=0))>>2]=0,wf[16+a>>2]=1,wf[12+a>>2]=1,wf[8+a>>2]=0,e=wf[(k?k+36|0:8+a|0)>>2],i=-2,4==(-4&(b=(b=wf[(k||24+a|0)>>2])||4))&&(i=wf[1200+(b<<2)>>2]),r=(b=i-1|0)- -8192|0,e=e?b&r:0,(f=4+(((Bf(i,i=(r>>>0)/(i>>>0)|0)+(wf[(k?k+8|0:16+a|0)>>2]<<2)|0)+e|0)+Bf((0!=(0|e))+i|0,4+(wf[(k?k+28|0:12+a|0)>>2]<<2)|0)|0)|0)>>>0>g[479]&&(B(wf[507]),wf[479]=f,wf[507]=S(f)),l)}function p(f){var r=0;if(Sf[f+16388>>1])If(f,0,16404);else{f:{r:{e:switch(Sf[f+16390>>1]){case 2:if((r=wf[f+16384>>2])>>>0<1073741825)break r;default:If(f,vf[f+16390>>1]=0,16388);break f;case 0:break e}r=wf[f+16384>>2]}r&&(wf[f+16384>>2]=r+65536)}wf[f+16400>>2]=0,wf[f+16392>>2]=0,wf[f+16396>>2]=0}}function y(f,r){var e,i=0,a=0;return r=I(r,(a=wf[f+262144>>2])-(i=(0|(i=a-(wf[f+262148>>2]+wf[f+262156>>2]|0)|0))<65536?i:65536)|0,i),a=wf[f+262144>>2],e=r+i|0,wf[f+262144>>2]=e,r=(a=a-wf[f+262148>>2]|0)-i|0,wf[f+262160>>2]=r,wf[f+262156>>2]=r,wf[f+262148>>2]=e-a,r>>>0>g[f+262164>>2]&&(wf[f+262164>>2]=r),i}function m(f,r){var e=0;gf[f+262171|0]?3&f||(wf[f+262172>>2]=0,wf[f+262144>>2]=-1,wf[f+262148>>2]=0,vf[f+262170>>1]=0):(wf[f+262172>>2]=0,e=wf[f+262148>>2],wf[f+262148>>2]=0,wf[f+262144>>2]=wf[f+262144>>2]-e),e=f,f=(0|r)<1?9:r,vf[e+262168>>1]=(0|f)<12?f:12}function C(f){return t((wf[f+4>>2]?((Cf(wf[f+12>>2],7)+Cf(wf[f+8>>2],1)|0)+Cf(wf[f+16>>2],12)|0)+Cf(wf[f+20>>2],18)|0:wf[f+16>>2]+374761393|0)+wf[f>>2]|0,f+24|0,wf[f+40>>2])}function R(f){wf[f+24>>2]=0,wf[f+28>>2]=0,wf[f+16>>2]=0,wf[f>>2]=0,wf[f+4>>2]=0,wf[f+20>>2]=1640531535,wf[f+12>>2]=-2048144777,wf[f+8>>2]=606290984,wf[f+32>>2]=0,wf[f+36>>2]=0,wf[f+40>>2]=0}function d(f){var r=0,e=0;return(f=(r=wf[480])+(e=f+3&-4)|0)>>>0<=r>>>0&&1<=(0|e)||f>>>0>(i.byteLength/65536|0)<<16>>>0&&!(0|a(0|f))?(wf[508]=48,-1):(wf[480]=f,r)}function v(f,r,e,i,a){var b=0;return f=function(f,r,e,i,a,b){var k,o=0,t=0,n=0,s=0,u=0,c=0,l=0,n=wf[f+262144>>2],s=wf[f+262148>>2];if(s||(1073741825<=n>>>0&&(If(If(f,0,131072)+131072|0,255,131072),n=0),wf[f+262144>>2]=r,o=n+65536|0,wf[f+262164>>2]=o,wf[f+262160>>2]=o,wf[f+262156>>2]=o,s=(r-n|0)-65536|0,wf[f+262148>>2]=s,wf[f+262152>>2]=s,n=r),o=n-s|0,!(o>>>0<2147483649||(u=(o=o-wf[f+262156>>2]|0)>>>0<65536?o:65536,t=vf[f+262168>>1],o=n,3&f||(s=0,vf[f+262170>>1]=0,wf[f+262172>>2]=0,o=-1),t=(0|t)<1?9:t,vf[f+262168>>1]=(0|t)<12?t:12,1073741825<=(o=o-s|0)>>>0&&(If(If(f,0,131072)+131072|0,255,131072),o=0),wf[f+262144>>2]=n,t=o+65536|0,wf[f+262160>>2]=t,wf[f+262156>>2]=t,wf[f+262164>>2]=t,s=((n-u|0)-o|0)-65536|0,wf[f+262152>>2]=s,wf[f+262148>>2]=s,(0|u)<4))){if(k=(n-s|0)-3|0,!(k>>>0<=t>>>0)&&(1&u||(c=(Bf(gf[0|(c=t+s|0)]|gf[c+1|0]<<8|(gf[c+2|0]<<16|gf[c+3|0]<<24),-1640531535)>>>15&131068)+f|0,l=t-wf[c>>2]|0,vf[131072+(((65535&o)<<1)+f|0)>>1]=l>>>0<65535?l:65535,wf[c>>2]=t,t=o+65537|0),4!=(0|u)))for(c=f+131072|0;o=(Bf(gf[0|(o=t+s|0)]|gf[o+1|0]<<8|(gf[o+2|0]<<16|gf[o+3|0]<<24),-1640531535)>>>15&131068)+f|0,u=t-wf[o>>2]|0,vf[c+((65535&t)<<1)>>1]=u>>>0<65535?u:65535,wf[o>>2]=t,u=(Bf(gf[0|(u=(o=t+1|0)+s|0)]|gf[u+1|0]<<8|(gf[u+2|0]<<16|gf[u+3|0]<<24),-1640531535)>>>15&131068)+f|0,l=o-wf[u>>2]|0,vf[c+((65535&o)<<1)>>1]=l>>>0<65535?l:65535,wf[u>>2]=o,(0|k)!=(0|(t=t+2|0)););wf[f+262164>>2]=k}if((0|r)!=(0|n)){if(o=wf[f+262156>>2],!(n>>>0<4+(s+o|0)>>>0||(u=(n-s|0)-3|0)>>>0<=(t=wf[f+262164>>2])>>>0)){for(;o=(Bf(gf[0|(o=t+s|0)]|gf[o+1|0]<<8|(gf[o+2|0]<<16|gf[o+3|0]<<24),-1640531535)>>>15&131068)+f|0,c=t-wf[o>>2]|0,vf[131072+(((65535&t)<<1)+f|0)>>1]=c>>>0<65535?c:65535,(t=(wf[o>>2]=t)+1|0)>>>0<u>>>0;);o=wf[f+262156>>2]}wf[f+262160>>2]=o,wf[f+262152>>2]=s,wf[f+262172>>2]=0,wf[f+262144>>2]=r,t=n-s|0,wf[f+262156>>2]=t,wf[f+262164>>2]=t,wf[f+262148>>2]=r-t}else t=wf[f+262156>>2],o=wf[f+262160>>2],s=wf[f+262152>>2];return(n=wf[i>>2]+r|0)>>>0<=o+s>>>0||(o=t+s|0)>>>0<=r>>>0||(n=(o>>>0<n>>>0?o:n)-s|0,wf[f+262160>>2]=t-n>>>0<4?t:n),n=vf[f+262168>>1],(wf[f+262172>>2]?A:Zf)(f,r,e,i,a,n,b)}(f,r,e,12+(_f=b=_f-16|0)|0,a,(0|a)<(0|((wf[12+b>>2]=i)>>>0<=2113929216?16+(((i>>>0)/255|0)+i|0)|0:0))),_f=16+b|0,f}function U(f,r,e,i,a,b){return b?(a+b|0)==(0|r)?65535<=(0|b)?function(f,r,e,i){var a,b,k,o,t,n,s,u,c,l=0,A=0,h=0,p=0,m=0,y=0,d=0,v=0,h=-1;f:if(f){if(!i){if(1!=(0|e))break f;return gf[0|f]?-1:0}if(e){n=(l=r+i|0)-32|0,s=(a=f+e|0)-16|0,u=l-5|0,b=l-7|0,k=a-5|0,c=a-8|0,o=l-12|0,t=a-15|0,p=f;r:{for(;;){l=p+1|0,e=r+m|0;e:{i:{if(15!=(0|(p=(y=gf[0|p])>>>4|0))){if(n>>>0<e>>>0|s>>>0<=l>>>0)break i;if(h=gf[l+4|0]|gf[l+5|0]<<8|(gf[l+6|0]<<16|gf[l+7|0]<<24),A=gf[0|l]|gf[l+1|0]<<8|(gf[l+2|0]<<16|gf[l+3|0]<<24),df[0|e]=A,df[e+1|0]=A>>>8,df[e+2|0]=A>>>16,df[e+3|0]=A>>>24,df[e+4|0]=h,df[e+5|0]=h>>>8,df[e+6|0]=h>>>16,df[e+7|0]=h>>>24,h=gf[l+12|0]|gf[l+13|0]<<8|(gf[l+14|0]<<16|gf[l+15|0]<<24),A=gf[l+8|0]|gf[l+9|0]<<8|(gf[l+10|0]<<16|gf[l+11|0]<<24),df[e+8|0]=A,df[e+9|0]=A>>>8,df[e+10|0]=A>>>16,df[e+11|0]=A>>>24,df[e+12|0]=h,df[e+13|0]=h>>>8,df[e+14|0]=h>>>16,df[e+15|0]=h>>>24,m=(h=p+m|0)-(d=gf[0|(e=l+p|0)]|gf[e+1|0]<<8)|0,l=p=e+2|0,15==(0|(e=15&y)))break e;if(l=p,d>>>0<8)break e;m=gf[(A=r+m|0)+4|0]|gf[A+5|0]<<8|(gf[A+6|0]<<16|gf[A+7|0]<<24),l=r+h|0,y=gf[0|A]|gf[A+1|0]<<8|(gf[A+2|0]<<16|gf[A+3|0]<<24),df[0|l]=y,df[l+1|0]=y>>>8,df[l+2|0]=y>>>16,df[l+3|0]=y>>>24,df[l+4|0]=m,df[l+5|0]=m>>>8,df[l+6|0]=m>>>16,df[l+7|0]=m>>>24,m=gf[A+12|0]|gf[A+13|0]<<8|(gf[A+14|0]<<16|gf[A+15|0]<<24),y=gf[A+8|0]|gf[A+9|0]<<8|(gf[A+10|0]<<16|gf[A+11|0]<<24),df[l+8|0]=y,df[l+9|0]=y>>>8,df[l+10|0]=y>>>16,df[l+11|0]=y>>>24,df[l+12|0]=m,df[l+13|0]=m>>>8,df[l+14|0]=m>>>16,df[l+15|0]=m>>>24,A=gf[A+16|0]|gf[A+17|0]<<8,df[l+16|0]=A,df[l+17|0]=A>>>8,m=4+(e+h|0)|0;continue}if(t>>>(p=0)<=l>>>0)break r;for(;p=(h=gf[0|l])+p|0,!(t>>>0<=(l=l+1|0)>>>0||255!=(0|h)););if((-1^e)>>>0<(p=p+15|0)>>>0|(-1^l)>>>0<p>>>0)break r}if(h=p+m|0,A=l+p|0,!(A>>>0<=c>>>0&&(d=r+h|0)>>>0<=o>>>0)){if((0|A)!=(0|a)|(0|i)<(0|h))break r;I(e,l,p);break f}for(;p=gf[l+4|0]|gf[l+5|0]<<8|(gf[l+6|0]<<16|gf[l+7|0]<<24),m=gf[0|l]|gf[l+1|0]<<8|(gf[l+2|0]<<16|gf[l+3|0]<<24),df[0|e]=m,df[e+1|0]=m>>>8,df[e+2|0]=m>>>16,df[e+3|0]=m>>>24,df[e+4|0]=p,df[e+5|0]=p>>>8,df[e+6|0]=p>>>16,df[e+7|0]=p>>>24,l=l+8|0,(e=e+8|0)>>>0<d>>>0;);e=15&y,m=h-(d=gf[0|A]|gf[A+1|0]<<8)|0,l=A+2|0}A=r+h|0;e:{if(15==(0|e)){for(y=k>>>0<l>>>0?l:k,e=0;;){if(p=l+1|0,(0|l)==(0|y))break e;if(e=(v=gf[0|l])+e|0,l=p,255!=(0|v))break}if((-1^A)>>>0<(e=e+15|0)>>>0)break r}else p=l;if(!((0|m)<-65536)){if(l=r+m|0,y=(m=(v=e+4|0)+h|0)+r|0,l=d>>>0<=7?(df[0|A]=0,df[A+1|0]=0,df[A+2|0]=0,df[A+3|0]=0,df[0|A]=gf[0|l],df[A+1|0]=gf[l+1|0],df[A+2|0]=gf[l+2|0],df[A+3|0]=gf[l+3|0],e=l+wf[(h=d<<2)+1024>>2]|0,l=gf[0|e]|gf[e+1|0]<<8|(gf[e+2|0]<<16|gf[e+3|0]<<24),df[A+4|0]=l,df[A+5|0]=l>>>8,df[A+6|0]=l>>>16,df[A+7|0]=l>>>24,e-wf[h+1056>>2]|0):(e=gf[l+4|0]|gf[l+5|0]<<8|(gf[l+6|0]<<16|gf[l+7|0]<<24),h=gf[0|l]|gf[l+1|0]<<8|(gf[l+2|0]<<16|gf[l+3|0]<<24),df[0|A]=h,df[A+1|0]=h>>>8,df[A+2|0]=h>>>16,df[A+3|0]=h>>>24,df[A+4|0]=e,df[A+5|0]=e>>>8,df[A+6|0]=e>>>16,df[A+7|0]=e>>>24,l+8|0),e=A+8|0,o>>>0<y>>>0){if(u>>>0<y>>>0)break e;if(h=l,(A=e)>>>0<b>>>0){for(;d=gf[h+4|0]|gf[h+5|0]<<8|(gf[h+6|0]<<16|gf[h+7|0]<<24),v=gf[0|h]|gf[h+1|0]<<8|(gf[h+2|0]<<16|gf[h+3|0]<<24),df[0|A]=v,df[A+1|0]=v>>>8,df[A+2|0]=v>>>16,df[A+3|0]=v>>>24,df[A+4|0]=d,df[A+5|0]=d>>>8,df[A+6|0]=d>>>16,df[A+7|0]=d>>>24,h=h+8|0,(A=A+8|0)>>>0<b>>>0;);l=(b-e|0)+l|0,e=b}if(y>>>0<=e>>>0)continue;for(;df[0|e]=gf[0|l],l=l+1|0,(0|y)!=(0|(e=e+1|0)););continue}if(h=gf[l+4|0]|gf[l+5|0]<<8|(gf[l+6|0]<<16|gf[l+7|0]<<24),d=gf[0|l]|gf[l+1|0]<<8|(gf[l+2|0]<<16|gf[l+3|0]<<24),df[0|e]=d,df[e+1|0]=d>>>8,df[e+2|0]=d>>>16,df[e+3|0]=d>>>24,df[e+4|0]=h,df[e+5|0]=h>>>8,df[e+6|0]=h>>>16,df[e+7|0]=h>>>24,v>>>0<17)continue;for(e=A+16|0;h=gf[l+12|0]|gf[l+13|0]<<8|(gf[l+14|0]<<16|gf[l+15|0]<<24),A=gf[l+8|0]|gf[l+9|0]<<8|(gf[l+10|0]<<16|gf[l+11|0]<<24),df[0|e]=A,df[e+1|0]=A>>>8,df[e+2|0]=A>>>16,df[e+3|0]=A>>>24,df[e+4|0]=h,df[e+5|0]=h>>>8,df[e+6|0]=h>>>16,df[e+7|0]=h>>>24,l=l+8|0,(e=e+8|0)>>>0<y>>>0;);continue}}break}l=p}return(-1^l)+f|0}}return h}(f,r,e,i):function(f,r,e,i,a){var b,k,o,t,n,s,u,c,l,A,h,p=0,m=0,y=0,d=0,v=0,w=0,m=-1;f:if(f){if(!i){if(1!=(0|e))break f;return gf[0|f]?-1:0}if(e){t=r-a|0,c=(b=r+i|0)-32|0,l=(k=f+e|0)-16|0,A=b-5|0,o=b-7|0,n=k-5|0,h=k-8|0,s=b-12|0,u=k-15|0,a=f,i=r;r:{for(;;){e:{e=a+1|0;i:{a:{b:{if(15!=(0|(m=(v=gf[0|a])>>>4|0))){if(c>>>0<i>>>0|l>>>0<=e>>>0)break b;if(p=gf[e+4|0]|gf[e+5|0]<<8|(gf[e+6|0]<<16|gf[e+7|0]<<24),a=gf[0|e]|gf[e+1|0]<<8|(gf[e+2|0]<<16|gf[e+3|0]<<24),df[0|i]=a,df[i+1|0]=a>>>8,df[i+2|0]=a>>>16,df[i+3|0]=a>>>24,df[i+4|0]=p,df[i+5|0]=p>>>8,df[i+6|0]=p>>>16,df[i+7|0]=p>>>24,p=gf[e+12|0]|gf[e+13|0]<<8|(gf[e+14|0]<<16|gf[e+15|0]<<24),a=gf[e+8|0]|gf[e+9|0]<<8|(gf[e+10|0]<<16|gf[e+11|0]<<24),df[i+8|0]=a,df[i+9|0]=a>>>8,df[i+10|0]=a>>>16,df[i+11|0]=a>>>24,df[i+12|0]=p,df[i+13|0]=p>>>8,df[i+14|0]=p>>>16,df[i+15|0]=p>>>24,p=(y=i+m|0)-(w=gf[0|(e=e+m|0)]|gf[e+1|0]<<8)|0,i=a=e+2|0,15==(0|(m=15&v)))break a;if(i=a,w>>>0<8)break a;if(p>>>0<t>>>0)break i;i=gf[p+4|0]|gf[p+5|0]<<8|(gf[p+6|0]<<16|gf[p+7|0]<<24),e=gf[0|p]|gf[p+1|0]<<8|(gf[p+2|0]<<16|gf[p+3|0]<<24),df[0|y]=e,df[y+1|0]=e>>>8,df[y+2|0]=e>>>16,df[y+3|0]=e>>>24,df[y+4|0]=i,df[y+5|0]=i>>>8,df[y+6|0]=i>>>16,df[y+7|0]=i>>>24,i=gf[p+12|0]|gf[p+13|0]<<8|(gf[p+14|0]<<16|gf[p+15|0]<<24),e=gf[p+8|0]|gf[p+9|0]<<8|(gf[p+10|0]<<16|gf[p+11|0]<<24),df[y+8|0]=e,df[y+9|0]=e>>>8,df[y+10|0]=e>>>16,df[y+11|0]=e>>>24,df[y+12|0]=i,df[y+13|0]=i>>>8,df[y+14|0]=i>>>16,df[y+15|0]=i>>>24,e=gf[p+16|0]|gf[p+17|0]<<8,df[y+16|0]=e,df[y+17|0]=e>>>8,i=4+(m+y|0)|0;continue}if(u>>>(m=0)<=e>>>0)break r;for(;m=(a=gf[0|e])+m|0,!(u>>>0<=(e=e+1|0)>>>0||255!=(0|a)););if((-1^i)>>>0<(m=m+15|0)>>>0|(-1^e)>>>0<m>>>0)break r}if(y=i+m|0,!((d=e+m|0)>>>0<=h>>>0&&y>>>0<=s>>>0)){if((0|d)!=(0|k)|b>>>0<y>>>0)break r;I(i,e,m),m=y-r|0;break f}for(;m=gf[e+4|0]|gf[e+5|0]<<8|(gf[e+6|0]<<16|gf[e+7|0]<<24),a=gf[0|e]|gf[e+1|0]<<8|(gf[e+2|0]<<16|gf[e+3|0]<<24),df[0|i]=a,df[i+1|0]=a>>>8,df[i+2|0]=a>>>16,df[i+3|0]=a>>>24,df[i+4|0]=m,df[i+5|0]=m>>>8,df[i+6|0]=m>>>16,df[i+7|0]=m>>>24,e=e+8|0,(i=i+8|0)>>>0<y>>>0;);m=15&v,p=y-(w=gf[0|d]|gf[d+1|0]<<8)|0,i=d+2|0}if(15==(0|m)){for(v=n>>>0<i>>>0?i:n,m=0;;){if(a=i+1|0,(0|i)==(0|v))break e;if(m=(e=gf[0|i])+m|0,i=a,255!=(0|e))break}if(e=i,(-1^y)>>>0<(m=m+15|0)>>>0)break r}else a=i}if(!(p>>>0<t>>>0)){if(i=(v=m+4|0)+y|0,e=w>>>0<=7?(df[0|y]=0,df[y+1|0]=0,df[y+2|0]=0,df[y+3|0]=0,df[0|y]=gf[0|p],df[y+1|0]=gf[p+1|0],df[y+2|0]=gf[p+2|0],df[y+3|0]=gf[p+3|0],p=wf[(e=w<<2)+1024>>2]+p|0,m=gf[0|p]|gf[p+1|0]<<8|(gf[p+2|0]<<16|gf[p+3|0]<<24),df[y+4|0]=m,df[y+5|0]=m>>>8,df[y+6|0]=m>>>16,df[y+7|0]=m>>>24,p-wf[e+1056>>2]|0):(m=gf[p+4|0]|gf[p+5|0]<<8|(gf[p+6|0]<<16|gf[p+7|0]<<24),e=gf[0|p]|gf[p+1|0]<<8|(gf[p+2|0]<<16|gf[p+3|0]<<24),df[0|y]=e,df[y+1|0]=e>>>8,df[y+2|0]=e>>>16,df[y+3|0]=e>>>24,df[y+4|0]=m,df[y+5|0]=m>>>8,df[y+6|0]=m>>>16,df[y+7|0]=m>>>24,p+8|0),m=y+8|0,s>>>0<i>>>0){if(A>>>0<i>>>0)break e;if(y=e,(p=m)>>>0<o>>>0){for(;d=gf[y+4|0]|gf[y+5|0]<<8|(gf[y+6|0]<<16|gf[y+7|0]<<24),v=gf[0|y]|gf[y+1|0]<<8|(gf[y+2|0]<<16|gf[y+3|0]<<24),df[0|p]=v,df[p+1|0]=v>>>8,df[p+2|0]=v>>>16,df[p+3|0]=v>>>24,df[p+4|0]=d,df[p+5|0]=d>>>8,df[p+6|0]=d>>>16,df[p+7|0]=d>>>24,y=y+8|0,(p=p+8|0)>>>0<o>>>0;);e=(o-m|0)+e|0,m=o}if(i>>>0<=m>>>0)continue;for(;df[0|m]=gf[0|e],e=e+1|0,(0|(m=m+1|0))!=(0|i););continue}if(d=gf[e+4|0]|gf[e+5|0]<<8|(gf[e+6|0]<<16|gf[e+7|0]<<24),p=gf[0|e]|gf[e+1|0]<<8|(gf[e+2|0]<<16|gf[e+3|0]<<24),df[0|m]=p,df[m+1|0]=p>>>8,df[m+2|0]=p>>>16,df[m+3|0]=p>>>24,df[m+4|0]=d,df[m+5|0]=d>>>8,df[m+6|0]=d>>>16,df[m+7|0]=d>>>24,v>>>0<17)continue;for(m=y+16|0;y=gf[e+12|0]|gf[e+13|0]<<8|(gf[e+14|0]<<16|gf[e+15|0]<<24),p=gf[e+8|0]|gf[e+9|0]<<8|(gf[e+10|0]<<16|gf[e+11|0]<<24),df[0|m]=p,df[m+1|0]=p>>>8,df[m+2|0]=p>>>16,df[m+3|0]=p>>>24,df[m+4|0]=y,df[m+5|0]=y>>>8,df[m+6|0]=y>>>16,df[m+7|0]=y>>>24,e=e+8|0,(m=m+8|0)>>>0<i>>>0;);continue}}break}e=a}return(-1^e)+f|0}}return m}(f,r,e,i,b):function(f,r,e,i,a,b){var k,o,t,n,s,u,c,l,A,h,p,m,y=0,d=0,v=0,w=0,g=0,v=-1;f:if(f){if(!i){if(1!=(0|e))break f;return gf[0|f]?-1:0}if(e){n=a?a+b|0:0,A=(k=r+i|0)-32|0,h=(o=f+e|0)-16|0,s=k-5|0,t=k-7|0,u=o-5|0,p=o-8|0,c=k-12|0,l=o-15|0,m=65535<b>>>0,a=f,i=r;r:{for(;;){e:{e=a+1|0;i:{a:{b:{if(15!=(0|(a=(w=gf[0|a])>>>4|0))){if(A>>>0<i>>>0|h>>>0<=e>>>0)break b;if(v=gf[e+4|0]|gf[e+5|0]<<8|(gf[e+6|0]<<16|gf[e+7|0]<<24),y=gf[0|e]|gf[e+1|0]<<8|(gf[e+2|0]<<16|gf[e+3|0]<<24),df[0|i]=y,df[i+1|0]=y>>>8,df[i+2|0]=y>>>16,df[i+3|0]=y>>>24,df[i+4|0]=v,df[i+5|0]=v>>>8,df[i+6|0]=v>>>16,df[i+7|0]=v>>>24,v=gf[e+12|0]|gf[e+13|0]<<8|(gf[e+14|0]<<16|gf[e+15|0]<<24),y=gf[e+8|0]|gf[e+9|0]<<8|(gf[e+10|0]<<16|gf[e+11|0]<<24),df[i+8|0]=y,df[i+9|0]=y>>>8,df[i+10|0]=y>>>16,df[i+11|0]=y>>>24,df[i+12|0]=v,df[i+13|0]=v>>>8,df[i+14|0]=v>>>16,df[i+15|0]=v>>>24,y=(d=i+a|0)-(g=gf[0|(e=e+a|0)]|gf[e+1|0]<<8)|0,a=e+2|0,15==(0|(v=15&w))){i=a;break a}if(g>>>0<8){i=a;break a}if(y>>>0<r>>>0)break i;e=gf[y+4|0]|gf[y+5|0]<<8|(gf[y+6|0]<<16|gf[y+7|0]<<24),i=gf[0|y]|gf[y+1|0]<<8|(gf[y+2|0]<<16|gf[y+3|0]<<24),df[0|d]=i,df[d+1|0]=i>>>8,df[d+2|0]=i>>>16,df[d+3|0]=i>>>24,df[d+4|0]=e,df[d+5|0]=e>>>8,df[d+6|0]=e>>>16,df[d+7|0]=e>>>24,e=gf[y+12|0]|gf[y+13|0]<<8|(gf[y+14|0]<<16|gf[y+15|0]<<24),i=gf[y+8|0]|gf[y+9|0]<<8|(gf[y+10|0]<<16|gf[y+11|0]<<24),df[d+8|0]=i,df[d+9|0]=i>>>8,df[d+10|0]=i>>>16,df[d+11|0]=i>>>24,df[d+12|0]=e,df[d+13|0]=e>>>8,df[d+14|0]=e>>>16,df[d+15|0]=e>>>24,e=gf[y+16|0]|gf[y+17|0]<<8,df[d+16|0]=e,df[d+17|0]=e>>>8,i=4+(d+v|0)|0;continue}if(l>>>(a=0)<=e>>>0)break r;for(;a=(v=gf[0|e])+a|0,!(l>>>0<=(e=e+1|0)>>>0||255!=(0|v)););if((-1^i)>>>0<(a=a+15|0)>>>0|(-1^e)>>>0<a>>>0)break r}if(d=i+a|0,!((y=e+a|0)>>>0<=p>>>0&&d>>>0<=c>>>0)){if((0|y)!=(0|o)|k>>>0<d>>>0)break r;I(i,e,a),v=d-r|0;break f}for(;a=gf[e+4|0]|gf[e+5|0]<<8|(gf[e+6|0]<<16|gf[e+7|0]<<24),v=gf[0|e]|gf[e+1|0]<<8|(gf[e+2|0]<<16|gf[e+3|0]<<24),df[0|i]=v,df[i+1|0]=v>>>8,df[i+2|0]=v>>>16,df[i+3|0]=v>>>24,df[i+4|0]=a,df[i+5|0]=a>>>8,df[i+6|0]=a>>>16,df[i+7|0]=a>>>24,e=e+8|0,(i=i+8|0)>>>0<d>>>0;);v=15&w,i=y+2|0,y=d-(g=gf[0|y]|gf[y+1|0]<<8)|0}if(15==(0|v)){for(e=u>>>0<i>>>0?i:u,v=0;;){if(a=i+1|0,(0|e)==(0|i))break e;if(v=(w=gf[0|i])+v|0,i=a,255!=(0|w))break}if(e=i,(-1^d)>>>0<(v=v+15|0)>>>0)break r}else a=i}if(!(b+y>>>0<r>>>0)||m){if(v=(w=v+4|0)+d|0,y>>>0<r>>>0){if(s>>>0<v>>>0)break e;if(w>>>0<=(y=r-y|0)>>>0){I(d,n-y|0,w),i=v;continue}if(e=w-y|0,(i=Ef(d,n-y|0,y)+y|0)-r>>>0<e>>>0){if(e=r,(0|w)<=(0|y))continue;for(;df[0|i]=gf[0|e],e=e+1|0,(i=i+1|0)>>>0<v>>>0;);continue}Ef(i,r,e),i=v;continue}if(e=g>>>0<=7?(df[0|d]=0,df[d+1|0]=0,df[d+2|0]=0,df[d+3|0]=0,df[0|d]=gf[0|y],df[d+1|0]=gf[y+1|0],df[d+2|0]=gf[y+2|0],df[d+3|0]=gf[y+3|0],e=(e=y)+wf[(y=g<<2)+1024>>2]|0,i=gf[0|e]|gf[e+1|0]<<8|(gf[e+2|0]<<16|gf[e+3|0]<<24),df[d+4|0]=i,df[d+5|0]=i>>>8,df[d+6|0]=i>>>16,df[d+7|0]=i>>>24,e-wf[y+1056>>2]|0):(e=gf[y+4|0]|gf[y+5|0]<<8|(gf[y+6|0]<<16|gf[y+7|0]<<24),i=gf[0|y]|gf[y+1|0]<<8|(gf[y+2|0]<<16|gf[y+3|0]<<24),df[0|d]=i,df[d+1|0]=i>>>8,df[d+2|0]=i>>>16,df[d+3|0]=i>>>24,df[d+4|0]=e,df[d+5|0]=e>>>8,df[d+6|0]=e>>>16,df[d+7|0]=e>>>24,y+8|0),y=d+8|0,c>>>0<v>>>0){if(s>>>0<v>>>0)break e;if(i=e,(d=y)>>>0<t>>>0){for(;w=gf[i+4|0]|gf[i+5|0]<<8|(gf[i+6|0]<<16|gf[i+7|0]<<24),g=gf[0|i]|gf[i+1|0]<<8|(gf[i+2|0]<<16|gf[i+3|0]<<24),df[0|d]=g,df[d+1|0]=g>>>8,df[d+2|0]=g>>>16,df[d+3|0]=g>>>24,df[d+4|0]=w,df[d+5|0]=w>>>8,df[d+6|0]=w>>>16,df[d+7|0]=w>>>24,i=i+8|0,(d=d+8|0)>>>0<t>>>0;);e=(t-y|0)+e|0,y=t}if((i=v)>>>0<=y>>>0)continue;for(;df[0|y]=gf[0|e],e=e+1|0,(0|(y=y+1|0))!=(0|i););continue}if(i=gf[e+4|0]|gf[e+5|0]<<8|(gf[e+6|0]<<16|gf[e+7|0]<<24),g=gf[0|e]|gf[e+1|0]<<8|(gf[e+2|0]<<16|gf[e+3|0]<<24),df[0|y]=g,df[y+1|0]=g>>>8,df[y+2|0]=g>>>16,df[y+3|0]=g>>>24,df[y+4|0]=i,df[y+5|0]=i>>>8,df[y+6|0]=i>>>16,df[y+7|0]=i>>>24,i=v,w>>>0<17)continue;for(i=d+16|0;y=gf[e+12|0]|gf[e+13|0]<<8|(gf[e+14|0]<<16|gf[e+15|0]<<24),d=gf[e+8|0]|gf[e+9|0]<<8|(gf[e+10|0]<<16|gf[e+11|0]<<24),df[0|i]=d,df[i+1|0]=d>>>8,df[i+2|0]=d>>>16,df[i+3|0]=d>>>24,df[i+4|0]=y,df[i+5|0]=y>>>8,df[i+6|0]=y>>>16,df[i+7|0]=y>>>24,e=e+8|0,(i=i+8|0)>>>0<v>>>0;);i=v;continue}}break}e=a}return(-1^e)+f|0}}return v}(f,r,e,i,a,b):function(f,r,e,i){var a,b,k,o,t,n,s,u,c,l,A=0,h=0,p=0,m=0,y=0,d=0,h=-1;f:if(f){if(!i){if(1!=(0|e))break f;return gf[0|f]?-1:0}if(e){s=(a=r+i|0)-32|0,u=(b=f+e|0)-16|0,c=a-5|0,k=a-7|0,o=b-5|0,l=b-8|0,t=a-12|0,n=b-15|0,m=f,i=r;r:{for(;;){e:{e=m+1|0;i:{a:{b:{if(15!=(0|(h=(d=gf[0|m])>>>4|0))){if(s>>>0<i>>>0|u>>>0<=e>>>0)break b;if(m=gf[e+4|0]|gf[e+5|0]<<8|(gf[e+6|0]<<16|gf[e+7|0]<<24),A=gf[0|e]|gf[e+1|0]<<8|(gf[e+2|0]<<16|gf[e+3|0]<<24),df[0|i]=A,df[i+1|0]=A>>>8,df[i+2|0]=A>>>16,df[i+3|0]=A>>>24,df[i+4|0]=m,df[i+5|0]=m>>>8,df[i+6|0]=m>>>16,df[i+7|0]=m>>>24,m=gf[e+12|0]|gf[e+13|0]<<8|(gf[e+14|0]<<16|gf[e+15|0]<<24),A=gf[e+8|0]|gf[e+9|0]<<8|(gf[e+10|0]<<16|gf[e+11|0]<<24),df[i+8|0]=A,df[i+9|0]=A>>>8,df[i+10|0]=A>>>16,df[i+11|0]=A>>>24,df[i+12|0]=m,df[i+13|0]=m>>>8,df[i+14|0]=m>>>16,df[i+15|0]=m>>>24,p=(A=i+h|0)-(y=gf[0|(e=e+h|0)]|gf[e+1|0]<<8)|0,i=m=e+2|0,15==(0|(h=15&d)))break a;if(i=m,y>>>0<8)break a;if(p>>>0<r>>>0)break i;e=gf[p+4|0]|gf[p+5|0]<<8|(gf[p+6|0]<<16|gf[p+7|0]<<24),i=gf[0|p]|gf[p+1|0]<<8|(gf[p+2|0]<<16|gf[p+3|0]<<24),df[0|A]=i,df[A+1|0]=i>>>8,df[A+2|0]=i>>>16,df[A+3|0]=i>>>24,df[A+4|0]=e,df[A+5|0]=e>>>8,df[A+6|0]=e>>>16,df[A+7|0]=e>>>24,e=gf[p+12|0]|gf[p+13|0]<<8|(gf[p+14|0]<<16|gf[p+15|0]<<24),i=gf[p+8|0]|gf[p+9|0]<<8|(gf[p+10|0]<<16|gf[p+11|0]<<24),df[A+8|0]=i,df[A+9|0]=i>>>8,df[A+10|0]=i>>>16,df[A+11|0]=i>>>24,df[A+12|0]=e,df[A+13|0]=e>>>8,df[A+14|0]=e>>>16,df[A+15|0]=e>>>24,e=gf[p+16|0]|gf[p+17|0]<<8,df[A+16|0]=e,df[A+17|0]=e>>>8,i=4+(A+h|0)|0;continue}if(n>>>(h=0)<=e>>>0)break r;for(;h=(m=gf[0|e])+h|0,!(n>>>0<=(e=e+1|0)>>>0||255!=(0|m)););if((-1^i)>>>0<(h=h+15|0)>>>0|(-1^e)>>>0<h>>>0)break r}if(A=i+h|0,!((m=e+h|0)>>>0<=l>>>0&&A>>>0<=t>>>0)){if((0|m)!=(0|b)|a>>>0<A>>>0)break r;I(i,e,h),h=A-r|0;break f}for(;h=gf[e+4|0]|gf[e+5|0]<<8|(gf[e+6|0]<<16|gf[e+7|0]<<24),p=gf[0|e]|gf[e+1|0]<<8|(gf[e+2|0]<<16|gf[e+3|0]<<24),df[0|i]=p,df[i+1|0]=p>>>8,df[i+2|0]=p>>>16,df[i+3|0]=p>>>24,df[i+4|0]=h,df[i+5|0]=h>>>8,df[i+6|0]=h>>>16,df[i+7|0]=h>>>24,e=e+8|0,(i=i+8|0)>>>0<A>>>0;);h=15&d,p=A-(y=gf[0|m]|gf[m+1|0]<<8)|0,i=m+2|0}if(15==(0|h)){for(e=o>>>0<i>>>0?i:o,h=0;;){if(m=i+1|0,(0|e)==(0|i))break e;if(h=(d=gf[0|i])+h|0,i=m,255!=(0|d))break}if(e=i,(-1^A)>>>0<(h=h+15|0)>>>0)break r}else m=i}if(!(p>>>0<r>>>0)){if(i=(d=h+4|0)+A|0,e=y>>>0<=7?(df[0|A]=0,df[A+1|0]=0,df[A+2|0]=0,df[A+3|0]=0,df[0|A]=gf[0|p],df[A+1|0]=gf[p+1|0],df[A+2|0]=gf[p+2|0],df[A+3|0]=gf[p+3|0],e=(e=p)+wf[(p=y<<2)+1024>>2]|0,h=gf[0|e]|gf[e+1|0]<<8|(gf[e+2|0]<<16|gf[e+3|0]<<24),df[A+4|0]=h,df[A+5|0]=h>>>8,df[A+6|0]=h>>>16,df[A+7|0]=h>>>24,e-wf[p+1056>>2]|0):(e=gf[p+4|0]|gf[p+5|0]<<8|(gf[p+6|0]<<16|gf[p+7|0]<<24),h=gf[0|p]|gf[p+1|0]<<8|(gf[p+2|0]<<16|gf[p+3|0]<<24),df[0|A]=h,df[A+1|0]=h>>>8,df[A+2|0]=h>>>16,df[A+3|0]=h>>>24,df[A+4|0]=e,df[A+5|0]=e>>>8,df[A+6|0]=e>>>16,df[A+7|0]=e>>>24,p+8|0),h=A+8|0,t>>>0<i>>>0){if(c>>>0<i>>>0)break e;if(A=e,(p=h)>>>0<k>>>0){for(;y=gf[A+4|0]|gf[A+5|0]<<8|(gf[A+6|0]<<16|gf[A+7|0]<<24),d=gf[0|A]|gf[A+1|0]<<8|(gf[A+2|0]<<16|gf[A+3|0]<<24),df[0|p]=d,df[p+1|0]=d>>>8,df[p+2|0]=d>>>16,df[p+3|0]=d>>>24,df[p+4|0]=y,df[p+5|0]=y>>>8,df[p+6|0]=y>>>16,df[p+7|0]=y>>>24,A=A+8|0,(p=p+8|0)>>>0<k>>>0;);e=(k-h|0)+e|0,h=k}if(i>>>0<=h>>>0)continue;for(;df[0|h]=gf[0|e],e=e+1|0,(0|(h=h+1|0))!=(0|i););continue}if(p=gf[e+4|0]|gf[e+5|0]<<8|(gf[e+6|0]<<16|gf[e+7|0]<<24),y=gf[0|e]|gf[e+1|0]<<8|(gf[e+2|0]<<16|gf[e+3|0]<<24),df[0|h]=y,df[h+1|0]=y>>>8,df[h+2|0]=y>>>16,df[h+3|0]=y>>>24,df[h+4|0]=p,df[h+5|0]=p>>>8,df[h+6|0]=p>>>16,df[h+7|0]=p>>>24,d>>>0<17)continue;for(h=A+16|0;A=gf[e+12|0]|gf[e+13|0]<<8|(gf[e+14|0]<<16|gf[e+15|0]<<24),p=gf[e+8|0]|gf[e+9|0]<<8|(gf[e+10|0]<<16|gf[e+11|0]<<24),df[0|h]=p,df[h+1|0]=p>>>8,df[h+2|0]=p>>>16,df[h+3|0]=p>>>24,df[h+4|0]=A,df[h+5|0]=A>>>8,df[h+6|0]=A>>>16,df[h+7|0]=A>>>24,e=e+8|0,(h=h+8|0)>>>0<i>>>0;);continue}}break}e=m}return(-1^e)+f|0}}return h}(f,r,e,i)}function w(f,r){var e=0,i=(e=wf[f+16400>>2])>>>0<65536?e:65536;return r=I(r,(e+wf[f+16392>>2]|0)-i|0,i),wf[f+16400>>2]=i,wf[f+16392>>2]=r,i}function J(f){var r=0,r=f;return!(f=S(r=(1|f)>>>0<65536?f:r))|!(3&gf[f-4|0])||If(f,0,r),f}function Cf(f,r){var e=0;return(-1>>>(e=31&r)&f)<<e|((e=f)&-1<<(f=0-r&31))>>>f}function Rf(f){return f?31-r(f-1^f)|0:32}M(e=gf,1028,"AQAAAAIAAAABAAAAAAAAAAQAAAAEAAAABA=="),M(e,1068,"//////z///8BAAAAAgAAAAMAAABVbnNwZWNpZmllZCBlcnJvciBjb2Rl"),M(e,1120,"0AQAANsEAADpBAAABAUAABwFAAA+BQAAXQUAAHcFAACTBQAAqgUAAMIFAADZBQAA8wUAABAGAAAoBgAAPgYAAFEGAABrBgAAiAYAAKYGAADJBg=="),M(e,1218,"AQAAAAQAAAAQAAAAQABPS19Ob0Vycm9yAEVSUk9SX0dFTkVSSUMARVJST1JfbWF4QmxvY2tTaXplX2ludmFsaWQARVJST1JfYmxvY2tNb2RlX2ludmFsaWQARVJST1JfY29udGVudENoZWNrc3VtRmxhZ19pbnZhbGlkAEVSUk9SX2NvbXByZXNzaW9uTGV2ZWxfaW52YWxpZABFUlJPUl9oZWFkZXJWZXJzaW9uX3dyb25nAEVSUk9SX2Jsb2NrQ2hlY2tzdW1faW52YWxpZABFUlJPUl9yZXNlcnZlZEZsYWdfc2V0AEVSUk9SX2FsbG9jYXRpb25fZmFpbGVkAEVSUk9SX3NyY1NpemVfdG9vTGFyZ2UARVJST1JfZHN0TWF4U2l6ZV90b29TbWFsbABFUlJPUl9mcmFtZUhlYWRlcl9pbmNvbXBsZXRlAEVSUk9SX2ZyYW1lVHlwZV91bmtub3duAEVSUk9SX2ZyYW1lU2l6ZV93cm9uZwBFUlJPUl9zcmNQdHJfd3JvbmcARVJST1JfZGVjb21wcmVzc2lvbkZhaWxlZABFUlJPUl9oZWFkZXJDaGVja3N1bV9pbnZhbGlkAEVSUk9SX2NvbnRlbnRDaGVja3N1bV9pbnZhbGlkAEVSUk9SX2ZyYW1lRGVjb2RpbmdfYWxyZWFkeVN0YXJ0ZWQARVJST1JfbWF4Q29kZQ=="),M(e,1764,"AgAAABAAAAAAAAAAAgAAABAAAAAAAAAAAgAAABAAAAAAAAAABAAAABAAAAAAAAAACAAAABAAAAAAAAAAEAAAABAAAAAAAAAAIAAAABAAAAAAAAAAQAAAABAAAAAAAAAAgAAAABAAAAAAAAAAAAEAABAAAAABAAAAYAAAAEAAAAABAAAAAAIAAIAAAAABAAAAAEAAAAAQ"),M(e,1917,"IAAA8AlQ");var F=((f=[null,function(f,r,e,i,a,b,k){f|=0,r|=0,e|=0,i|=0,a|=0,k|=0;var o=(0|(b|=0))<0,t=1-b|0;return(0|b)<=2?k?(p(f),s(f,wf[k+4>>2])):s(f,0):(m(f,b),wf[f+262172>>2]=k?wf[k+8>>2]:0),b=o?t:1,k?0|n(f,r,e,i,a,b):0|function(f,r,e,i,a,b){var k=0,o=0,t=0,n=0,s=0,u=0,c=0,l=0,A=0,h=0,p=0,m=0,y=0,d=0,v=0,w=0,g=0,S=0,k=1<(0|b)?b:1;f:{r:{e:{i:{a:{b:{if((0|(i>>>0<=2113929216?16+(((i>>>0)/255|0)+i|0)|0:0))<=(0|a)){if(65546<(0|i))break b;if(Sf[f+16388>>1]){If(f,0,16404);break a}k:{o:{t:switch(Sf[f+16390>>1]){case 0:s=wf[f+16384>>2];break k;case 3:break t;default:break o}if(!(4095<(0|i))&&(s=wf[f+16384>>2])+i>>>0<65535)break k}s=0,If(f,vf[f+16390>>1]=0,16388)}if(wf[f+16400>>2]=0,wf[f+16392>>2]=0,wf[f+16396>>2]=0,!s)break a;if(2113929216<i>>>0)break i;l=r+i|0,vf[f+16390>>1]=3,wf[f+16400>>2]=i,wf[f+16384>>2]=i+s;k:if((0|i)<13)t=e,k=r;else for(h=r-s|0,A=l-11|0,vf[(Bf(gf[0|r]|gf[r+1|0]<<8|(gf[r+2|0]<<16|gf[r+3|0]<<24),-1640531535)>>>18&16382)+f>>1]=s,a=r+1|0,b=r+2|0,v=(m=l-5|0)-1|0,y=m-3|0,c=1|(p=k<<6),k=r,t=e;;){for(u=gf[0|a]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24),n=p,i=c;d=(Bf(u,-1640531535)>>>18&16382)+f|0,o=Sf[d>>1],u=gf[0|b]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24),vf[d>>1]=a-h,!(s>>>0<=o>>>0&&(gf[0|(o=o+h|0)]|gf[o+1|0]<<8|(gf[o+2|0]<<16|gf[o+3|0]<<24))==(gf[0|a]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24)));)if(o=n>>6,i=(n=i)+1|0,!((b=(a=b)+o|0)>>>0<=A>>>0))break k;for(;!((b=o)>>>0<=r>>>0|(n=a)>>>0<=k>>>0||(o=b-1|0,gf[0|(a=n-1|0)]!=gf[0|o])););for(i=t+1|0,15<=(a=n-k|0)>>>0?(df[0|t]=240,255<=(0|(o=a-15|0))&&(If(g=i,255,(i=(239+(n-(((0|o)<509?o:509)+k|0)|0)>>>0)/255|0)+1|0),o=(a+Bf(i,-255)|0)-270|0,i=2+(i+t|0)|0),df[0|i]=o,i=i+1|0):df[0|t]=a<<4,o=i+a|0;a=gf[k+4|0]|gf[k+5|0]<<8|(gf[k+6|0]<<16|gf[k+7|0]<<24),u=gf[0|k]|gf[k+1|0]<<8|(gf[k+2|0]<<16|gf[k+3|0]<<24),df[0|i]=u,df[i+1|0]=u>>>8,df[i+2|0]=u>>>16,df[i+3|0]=u>>>24,df[i+4|0]=a,df[i+5|0]=a>>>8,df[i+6|0]=a>>>16,df[i+7|0]=a>>>24,k=k+8|0,(i=i+8|0)>>>0<o>>>0;);for(k=n;;){n=t,i=k-b|0,df[0|o]=i,df[o+1|0]=i>>>8,i=b+4|0;o:{t:{if((t=y)>>>0<=(a=k+4|0)>>>0)b=a;else{if(i=(gf[0|a]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24))^(gf[0|i]|gf[i+1|0]<<8|(gf[i+2|0]<<16|gf[i+3|0]<<24)))break t;i=b+8|0,b=k+8|0}if(b>>>0<t>>>0)for(;;){if(t=(gf[0|b]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24))^(gf[0|i]|gf[i+1|0]<<8|(gf[i+2|0]<<16|gf[i+3|0]<<24))){i=((Rf(t)>>>3|0)+b|0)-a|0;break o}if(i=i+4|0,!((b=b+4|0)>>>0<y>>>0))break}(gf[0|i]|gf[i+1|0]<<8)!=(gf[0|b]|gf[b+1|0]<<8)|v>>>0<=b>>>0||(b=b+2|0,i=i+2|0),b>>>0<m>>>0&&(b=gf[0|i]==gf[0|b]?b+1|0:b),i=b-a|0;break o}i=Rf(i)>>>3|0}if(t=o+2|0,k=4+(i+k|0)|0,a=gf[0|n],15<=i>>>0?(df[0|n]=a+15,df[0|t]=255,df[t+1|0]=255,df[t+2|0]=255,df[t+3|0]=255,1020<=(b=i-15|0)>>>0&&(b=Bf(a=((i=i-1035|0)>>>0)/1020|0,-1020)+i|0,t=If(o+6|0,255,(i=a<<2)+4|0)+i|0),df[0|(a=t+(i=((65535&b)>>>0)/255|0)|0)]=i+b,t=a+1|0):df[0|n]=i+a,A>>>0<=k>>>0)break k;if(vf[(Bf(gf[0|(i=k-2|0)]|gf[i+1|0]<<8|(gf[i+2|0]<<16|gf[i+3|0]<<24),-1640531535)>>>18&16382)+f>>1]=i-h,a=(Bf(gf[0|k]|gf[k+1|0]<<8|(gf[k+2|0]<<16|gf[k+3|0]<<24),-1640531535)>>>18&16382)+f|0,i=Sf[a>>1],vf[a>>1]=k-h,i>>>0<s>>>0||(gf[0|(b=i+h|0)]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24))!=(gf[0|k]|gf[k+1|0]<<8|(gf[k+2|0]<<16|gf[k+3|0]<<24)))break;o=t+1|(df[0|t]=0)}if(a=k+1|0,!((b=k+2|0)>>>0<=A>>>0))break}15<=(r=l-k|0)>>>0?(df[0|t]=240,f=t+1|0,(i=r-15|0)>>>0<255?df[0|(t=f)]=i:(If(a=f,255,(f=((i=r-270|0)>>>0)/255|0)+1|0),df[0|(t=2+(f+t|0)|0)]=i+Bf(f,-255))):df[0|t]=r<<4;break r}k:{if((0|i)<=65546){if(Sf[f+16388>>1]){If(f,0,16404);break k}o:{t:{n:switch(Sf[f+16390>>1]){case 0:s=wf[f+16384>>2];break o;case 3:break n;default:break t}if(!(4095<(0|i))&&(s=wf[f+16384>>2])+i>>>0<65535)break o}s=0,If(f,vf[f+16390>>1]=0,16388)}if(wf[f+16400>>2]=0,wf[f+16392>>2]=0,wf[f+16396>>2]=0,!s)break k;if(2113929216<i>>>0)break i;y=e+a|0,m=r+i|0,vf[f+16390>>1]=3,wf[f+16400>>2]=i,wf[f+16384>>2]=i+s;o:if((0|i)<13)t=e,k=r;else for(A=r-s|0,v=m-11|0,vf[(Bf(gf[0|r]|gf[r+1|0]<<8|(gf[r+2|0]<<16|gf[r+3|0]<<24),-1640531535)>>>18&16382)+f>>1]=s,a=r+1|0,b=r+2|0,S=(w=m-5|0)-1|0,p=w-3|0,h=1|(c=k<<6),k=r,t=e;;){for(u=gf[0|a]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24),n=c,i=h;l=(Bf(u,-1640531535)>>>18&16382)+f|0,o=Sf[l>>1],u=gf[0|b]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24),vf[l>>1]=a-A,!(s>>>0<=o>>>0&&(gf[0|(o=o+A|0)]|gf[o+1|0]<<8|(gf[o+2|0]<<16|gf[o+3|0]<<24))==(gf[0|a]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24)));)if(o=n>>6,i=(n=i)+1|0,!((b=(a=b)+o|0)>>>0<=v>>>0))break o;for(;!((b=o)>>>0<=r>>>0|(n=a)>>>0<=k>>>0||(o=b-1|0,gf[0|(a=n-1|0)]!=gf[0|o])););if(y>>>0<9+(((a=n-k|0)+t|0)+((a>>>0)/255|0)|0)>>>0)return 0;for(i=t+1|0,15<=a>>>0?(df[0|t]=240,255<=(0|(o=a-15|0))&&(If(g=i,255,(i=(239+(n-(((0|o)<509?o:509)+k|0)|0)>>>0)/255|0)+1|0),o=(a+Bf(i,-255)|0)-270|0,i=2+(i+t|0)|0),df[0|i]=o,i=i+1|0):df[0|t]=a<<4,o=i+a|0;a=gf[k+4|0]|gf[k+5|0]<<8|(gf[k+6|0]<<16|gf[k+7|0]<<24),u=gf[0|k]|gf[k+1|0]<<8|(gf[k+2|0]<<16|gf[k+3|0]<<24),df[0|i]=u,df[i+1|0]=u>>>8,df[i+2|0]=u>>>16,df[i+3|0]=u>>>24,df[i+4|0]=a,df[i+5|0]=a>>>8,df[i+6|0]=a>>>16,df[i+7|0]=a>>>24,k=k+8|0,(i=i+8|0)>>>0<o>>>0;);for(k=n;;){i=k-b|0,df[0|o]=i,df[o+1|0]=i>>>8,i=b+4|0;t:{n:{if((l=p)>>>0<=(a=k+4|0)>>>0)b=a;else{if(i=(gf[0|a]|gf[a+1|0]<<8|(gf[a+2|0]<<16|gf[a+3|0]<<24))^(gf[0|i]|gf[i+1|0]<<8|(gf[i+2|0]<<16|gf[i+3|0]<<24)))break n;i=b+8|0,b=k+8|0}if(b>>>0<l>>>0)for(;;){if(l=(gf[0|b]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24))^(gf[0|i]|gf[i+1|0]<<8|(gf[i+2|0]<<16|gf[i+3|0]<<24))){i=((Rf(l)>>>3|0)+b|0)-a|0;break t}if(i=i+4|0,!((b=b+4|0)>>>0<p>>>0))break}(gf[0|i]|gf[i+1|0]<<8)!=(gf[0|b]|gf[b+1|0]<<8)|S>>>0<=b>>>0||(b=b+2|0,i=i+2|0),b>>>0<w>>>0&&(b=gf[0|i]==gf[0|b]?b+1|0:b),i=b-a|0;break t}i=Rf(i)>>>3|0}if(y>>>0<8+(((i+240>>>0)/255|0)+o|0)>>>0)return 0;if(a=t,t=o+2|0,k=4+(i+k|0)|0,b=gf[0|a],15<=i>>>0?(df[0|a]=b+15,df[0|t]=255,df[t+1|0]=255,df[t+2|0]=255,df[t+3|0]=255,1020<=(b=i-15|0)>>>0&&(b=Bf(a=((i=i-1035|0)>>>0)/1020|0,-1020)+i|0,t=If(o+6|0,255,(i=a<<2)+4|0)+i|0),df[0|(a=t+(i=((65535&b)>>>0)/255|0)|0)]=i+b,t=a+1|0):df[0|a]=i+b,v>>>0<=k>>>0)break o;if(vf[(Bf(gf[0|(i=k-2|0)]|gf[i+1|0]<<8|(gf[i+2|0]<<16|gf[i+3|0]<<24),-1640531535)>>>18&16382)+f>>1]=i-A,a=(Bf(gf[0|k]|gf[k+1|0]<<8|(gf[k+2|0]<<16|gf[k+3|0]<<24),-1640531535)>>>18&16382)+f|0,i=Sf[a>>1],vf[a>>1]=k-A,i>>>0<s>>>0||(gf[0|(b=i+A|0)]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24))!=(gf[0|k]|gf[k+1|0]<<8|(gf[k+2|0]<<16|gf[k+3|0]<<24)))break;o=t+1|(df[0|t]=0)}if(a=k+1|0,!((b=k+2|0)>>>0<=v>>>0))break}if(y>>>0<1+((t+(r=m-k|0)|0)+((r+240>>>0)/255|0)|0)>>>0)break i;15<=r>>>0?(df[0|t]=240,f=t+1|0,(i=r-15|0)>>>0<255?df[0|(t=f)]=i:(If(a=f,255,(f=((i=r-270|0)>>>0)/255|0)+1|0),df[0|(t=2+(f+t|0)|0)]=i+Bf(f,-255))):df[0|t]=r<<4;break r}if(Sf[f+16388>>1]?If(f,b=0,16404):(Sf[f+16390>>1]?(b=0,If(f,vf[f+16390>>1]=0,16388)):!(b=wf[f+16384>>2])|65535<r>>>0||(b=b+65536|0,wf[f+16384>>2]=b),wf[f+16400>>2]=0,wf[f+16392>>2]=0,wf[f+16396>>2]=0),2113929216<i>>>0)break i;l=(m=r+i|0)-5|0,wf[f+16400>>2]=i,wf[f+16384>>2]=i+b,vf[f+16390>>1]=65535<r>>>0?1:2,i=Bf(gf[0|r]|gf[r+1|0]<<8|(gf[r+2|0]<<16|gf[r+3|0]<<24),-1640531535)>>>20|0,wf[(i<<2)+f>>2]=65536<=r>>>0?r:b,A=e+a|0,h=r-b|0,u=m-11|0,v=l-1|0,c=l-3|0,p=1|(y=k<<6),w=65535<r>>>0,S=r>>>0<65536,t=e,k=r;o:for(;;){t:{b=k+2|0,o=Bf(gf[i=k+1|0]|gf[k+2|0]<<8|(gf[k+3|0]<<16|gf[k+4|0]<<24),-1640531535)>>>20|0;n:{if(!w){if(s=y,a=p,u>>>0<b>>>0)break t;for(;;){if(n=(o<<2)+f|0,o=wf[n>>2],d=gf[0|b]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24),g=n,n=i-h|0,wf[g>>2]=n,n>>>0<=o+65535>>>0&&(gf[0|(o=o+h|0)]|gf[o+1|0]<<8|(gf[o+2|0]<<16|gf[o+3|0]<<24))==(gf[0|i]|gf[i+1|0]<<8|(gf[i+2|0]<<16|gf[i+3|0]<<24)))break n;if(n=s>>6,o=Bf(d,-1640531535)>>>20|0,a=(s=a)+1|0,!((b=n+(i=b)|0)>>>0<=u>>>0))break}break t}if(n=y,a=p,u>>>0<b>>>0)break t;for(;;){if(o=wf[(s=(o<<2)+f|0)>>2],d=gf[0|b]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24),wf[s>>2]=i,(gf[0|o]|gf[o+1|0]<<8|(gf[o+2|0]<<16|gf[o+3|0]<<24))==(gf[0|i]|gf[i+1|0]<<8|(gf[i+2|0]<<16|gf[i+3|0]<<24))&&i>>>0<=o+65535>>>0)break n;if(s=n>>6,o=Bf(d,-1640531535)>>>20|0,a=(n=a)+1|0,!((b=s+(i=b)|0)>>>0<=u>>>0))break}break t}for(;!((b=o)>>>0<=r>>>0|(n=i)>>>0<=k>>>0||(o=b-1|0,gf[0|(i=n-1|0)]!=gf[0|o])););if(A>>>(d=0)<9+(((o=n-k|0)+t|0)+((o>>>0)/255|0)|0)>>>0)break i;for(i=t+1|0,15<=o>>>0?(df[0|t]=240,255<=(0|(a=o-15|0))&&(If(s=i,255,(i=(239+(n-(((0|a)<509?a:509)+k|0)|0)>>>0)/255|0)+1|0),a=(o+Bf(i,-255)|0)-270|0,i=2+(i+t|0)|0),df[0|i]=a,i=i+1|0):df[0|t]=o<<4,a=i+o|0;o=gf[k+4|0]|gf[k+5|0]<<8|(gf[k+6|0]<<16|gf[k+7|0]<<24),s=gf[0|k]|gf[k+1|0]<<8|(gf[k+2|0]<<16|gf[k+3|0]<<24),df[0|i]=s,df[i+1|0]=s>>>8,df[i+2|0]=s>>>16,df[i+3|0]=s>>>24,df[i+4|0]=o,df[i+5|0]=o>>>8,df[i+6|0]=o>>>16,df[i+7|0]=o>>>24,k=k+8|0,(i=i+8|0)>>>0<a>>>0;);for(k=n;;){i=k-b|0,df[0|a]=i,df[a+1|0]=i>>>8,i=b+4|0,n=A;n:{s:{if((s=c)>>>0<=(o=k+4|0)>>>0)b=o;else{if(i=(gf[0|o]|gf[o+1|0]<<8|(gf[o+2|0]<<16|gf[o+3|0]<<24))^(gf[0|i]|gf[i+1|0]<<8|(gf[i+2|0]<<16|gf[i+3|0]<<24)))break s;i=b+8|0,b=k+8|0}if(b>>>0<s>>>0)for(;;){if(s=(gf[0|b]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24))^(gf[0|i]|gf[i+1|0]<<8|(gf[i+2|0]<<16|gf[i+3|0]<<24))){i=((Rf(s)>>>3|0)+b|0)-o|0;break n}if(i=i+4|0,!((b=b+4|0)>>>0<c>>>0))break}(gf[0|i]|gf[i+1|0]<<8)!=(gf[0|b]|gf[b+1|0]<<8)|v>>>0<=b>>>0||(b=b+2|0,i=i+2|0),b>>>0<l>>>0&&(b=gf[0|i]==gf[0|b]?b+1|0:b),i=b-o|0;break n}i=Rf(i)>>>3|0}if(n>>>0<8+(((i+240>>>0)/255|0)+a|0)>>>0)break i;if(b=a+2|0,k=4+(i+k|0)|0,o=gf[0|t],15<=i>>>0?(df[0|t]=o+15,df[0|b]=255,df[b+1|0]=255,df[b+2|0]=255,df[b+3|0]=255,1020<=(o=i-15|0)>>>0&&(o=Bf(b=((i=i-1035|0)>>>0)/1020|0,-1020)+i|0,b=If(a+6|0,255,(i=b<<2)+4|0)+i|0),df[0|(a=b+(i=((65535&o)>>>0)/255|0)|0)]=i+o,b=a+1|0):df[0|t]=i+o,t=b,u>>>0<=k>>>0)break t;if(a=Bf(gf[0|(i=k-2|0)]|gf[i+1|0]<<8|(gf[i+2|0]<<16|gf[i+3|0]<<24),-1640531535)>>>20|0,S){if(wf[(a<<2)+f>>2]=i-h,a=(Bf(gf[0|k]|gf[k+1|0]<<8|(gf[k+2|0]<<16|gf[k+3|0]<<24),-1640531535)>>>18&16380)+f|0,i=wf[a>>2],b=a,a=k-h|0,wf[b>>2]=a,b=i+h|0,i+65535>>>0<a>>>0)continue o}else if(wf[(a<<2)+f>>2]=i,i=(Bf(gf[0|k]|gf[k+1|0]<<8|(gf[k+2|0]<<16|gf[k+3|0]<<24),-1640531535)>>>18&16380)+f|0,(b=wf[i>>2])+65535>>>0<(wf[i>>2]=k)>>>0)continue o;if((gf[0|b]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24))!=(gf[0|k]|gf[k+1|0]<<8|(gf[k+2|0]<<16|gf[k+3|0]<<24)))continue o;a=t+1|(df[0|t]=0)}}break}if(A>>>(d=0)<1+(((r=m-k|0)+t|0)+((r+240>>>0)/255|0)|0)>>>0)break i;15<=r>>>0?(df[0|t]=240,f=t+1|0,(i=r-15|0)>>>0<255?df[0|(t=f)]=i:(If(a=f,255,(f=((i=r-270|0)>>>0)/255|0)+1|0),df[0|(t=2+(f+t|0)|0)]=i+Bf(f,-255))):df[0|t]=r<<4,d=(Ef(t+1|0,k,r)+r|0)-e|0;break i}if(2113929216<i>>>0)break i;s=e+a|0,l=r+i|0,vf[f+16390>>1]=3,wf[f+16384>>2]=i;k:if((0|(wf[f+16400>>2]=i))<13)b=e,k=r;else{A=l-11|0,v=(m=l-5|(vf[(Bf(gf[0|r]|gf[r+1|0]<<8|(gf[r+2|0]<<16|gf[r+3|0]<<24),-1640531535)>>>18&16382)+f>>1]=0))-1|0,y=m-3|0,p=k<<6,b=e,k=r;o:for(;;){for(c=gf[o=k+1|0]|gf[k+2|0]<<8|(gf[k+3|0]<<16|gf[k+4|0]<<24),u=1,a=p;;){if(A>>>0<(o=u+(i=o)|0)>>>0)break k;if(t=Bf(c,-1640531535),c=gf[0|o]|gf[o+1|0]<<8|(gf[o+2|0]<<16|gf[o+3|0]<<24),n=Sf[(t=(t>>>18&16382)+f|0)>>1],vf[t>>1]=i-r,u=a>>6,a=a+1|0,(gf[0|(t=r+n|0)]|gf[t+1|0]<<8|(gf[t+2|0]<<16|gf[t+3|0]<<24))==(gf[0|i]|gf[i+1|0]<<8|(gf[i+2|0]<<16|gf[i+3|0]<<24)))break}for(;!((o=t)>>>0<=r>>>0|(n=i)>>>0<=k>>>0||(t=o-1|0,gf[0|(i=n-1|0)]!=gf[0|t])););if(s>>>0<9+(((a=n-k|0)+b|0)+((a>>>0)/255|0)|0)>>>0)return 0;for(i=b+1|0,15<=a>>>0?(df[0|b]=240,255<=(0|(c=a-15|0))&&(If(t=i,255,(i=(239+(n-(((0|c)<509?c:509)+k|0)|0)>>>0)/255|0)+1|0),c=(a+Bf(i,-255)|0)-270|0,i=2+(i+b|0)|0),df[0|i]=c,i=i+1|0):df[0|b]=a<<4,a=i+a|0;t=gf[k+4|0]|gf[k+5|0]<<8|(gf[k+6|0]<<16|gf[k+7|0]<<24),c=gf[0|k]|gf[k+1|0]<<8|(gf[k+2|0]<<16|gf[k+3|0]<<24),df[0|i]=c,df[i+1|0]=c>>>8,df[i+2|0]=c>>>16,df[i+3|0]=c>>>24,df[i+4|0]=t,df[i+5|0]=t>>>8,df[i+6|0]=t>>>16,df[i+7|0]=t>>>24,k=k+8|0,(i=i+8|0)>>>0<a>>>0;);for(k=n;;){n=b,i=k-o|0,df[0|a]=i,df[a+1|0]=i>>>8,i=o+4|0;t:{n:{if((u=y)>>>0<=(t=k+4|0)>>>0)b=t;else{if(i=(gf[0|t]|gf[t+1|0]<<8|(gf[t+2|0]<<16|gf[t+3|0]<<24))^(gf[0|i]|gf[i+1|0]<<8|(gf[i+2|0]<<16|gf[i+3|0]<<24)))break n;i=o+8|0,b=k+8|0}if(b>>>0<u>>>0)for(;;){if(o=(gf[0|b]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24))^(gf[0|i]|gf[i+1|0]<<8|(gf[i+2|0]<<16|gf[i+3|0]<<24))){i=((Rf(o)>>>3|0)+b|0)-t|0;break t}if(i=i+4|0,!((b=b+4|0)>>>0<y>>>0))break}(gf[0|i]|gf[i+1|0]<<8)!=(gf[0|b]|gf[b+1|0]<<8)|v>>>0<=b>>>0||(b=b+2|0,i=i+2|0),b>>>0<m>>>0&&(b=gf[0|i]==gf[0|b]?b+1|0:b),i=b-t|0;break t}i=Rf(i)>>>3|0}if(s>>>0<8+(((i+240>>>0)/255|0)+a|0)>>>0)return 0;if(b=a+2|0,k=4+(i+k|0)|0,t=gf[0|n],15<=i>>>0?(df[0|n]=t+15,df[0|b]=255,df[b+1|0]=255,df[b+2|0]=255,df[b+3|0]=255,1020<=(o=i-15|0)>>>0&&(o=Bf(b=((i=i-1035|0)>>>0)/1020|0,-1020)+i|0,b=If(a+6|0,255,(i=b<<2)+4|0)+i|0),df[0|(a=b+(i=((65535&o)>>>0)/255|0)|0)]=i+o,b=a+1|0):df[0|n]=i+t,A>>>0<=k>>>0)break k;if(vf[(Bf(gf[0|(i=k-2|0)]|gf[i+1|0]<<8|(gf[i+2|0]<<16|gf[i+3|0]<<24),-1640531535)>>>18&16382)+f>>1]=i-r,i=(Bf(gf[0|k]|gf[k+1|0]<<8|(gf[k+2|0]<<16|gf[k+3|0]<<24),-1640531535)>>>18&16382)+f|0,a=Sf[i>>1],vf[i>>1]=k-r,(gf[0|(o=r+a|0)]|gf[o+1|0]<<8|(gf[o+2|0]<<16|gf[o+3|0]<<24))!=(gf[0|k]|gf[k+1|0]<<8|(gf[k+2|0]<<16|gf[k+3|0]<<24)))continue o;a=b+1|(df[0|b]=0)}}}if(s>>>0<1+(((r=l-k|0)+b|0)+((r+240>>>0)/255|0)|0)>>>0)break i;15<=r>>>0?(df[0|b]=240,f=b+1|0,(i=r-15|0)>>>0<255?df[0|(b=f)]=i:(If(a=f,255,(f=((i=r-270|0)>>>0)/255|0)+1|0),df[0|(b=2+(f+b|0)|0)]=i+Bf(f,-255))):df[0|b]=r<<4;break f}if(Sf[f+16388>>1]?If(f,b=0,16404):(Sf[f+16390>>1]?(b=0,If(f,vf[f+16390>>1]=0,16388)):!(b=wf[f+16384>>2])|65535<r>>>0||(b=b+65536|0,wf[f+16384>>2]=b),wf[f+16400>>2]=0,wf[f+16392>>2]=0,wf[f+16396>>2]=0),2113929216<i>>>0)break i;l=(A=r+i|0)-5|0,wf[f+16400>>2]=i,wf[f+16384>>2]=i+b,vf[f+16390>>1]=65535<r>>>0?1:2,i=Bf(gf[0|r]|gf[r+1|0]<<8|(gf[r+2|0]<<16|gf[r+3|0]<<24),-1640531535)>>>20|0,wf[(i<<2)+f>>2]=65536<=r>>>0?r:b,h=r-b|0,u=A-11|0,m=l-1|0,c=l-3|0,p=1|(y=k<<6),v=65535<r>>>0,d=r>>>0<65536,t=e,k=r;b:for(;;){k:{b=k+2|0,o=Bf(gf[i=k+1|0]|gf[k+2|0]<<8|(gf[k+3|0]<<16|gf[k+4|0]<<24),-1640531535)>>>20|0;o:{if(!v){if(s=y,a=p,u>>>0<b>>>0)break k;for(;;){if(n=(o<<2)+f|0,o=wf[n>>2],w=gf[0|b]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24),g=n,n=i-h|0,wf[g>>2]=n,n>>>0<=o+65535>>>0&&(gf[0|(o=o+h|0)]|gf[o+1|0]<<8|(gf[o+2|0]<<16|gf[o+3|0]<<24))==(gf[0|i]|gf[i+1|0]<<8|(gf[i+2|0]<<16|gf[i+3|0]<<24)))break o;if(n=s>>6,o=Bf(w,-1640531535)>>>20|0,a=(s=a)+1|0,!((b=n+(i=b)|0)>>>0<=u>>>0))break}break k}if(n=y,a=p,u>>>0<b>>>0)break k;for(;;){if(o=wf[(s=(o<<2)+f|0)>>2],w=gf[0|b]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24),wf[s>>2]=i,(gf[0|o]|gf[o+1|0]<<8|(gf[o+2|0]<<16|gf[o+3|0]<<24))==(gf[0|i]|gf[i+1|0]<<8|(gf[i+2|0]<<16|gf[i+3|0]<<24))&&i>>>0<=o+65535>>>0)break o;if(s=n>>6,o=Bf(w,-1640531535)>>>20|0,a=(n=a)+1|0,!((b=s+(i=b)|0)>>>0<=u>>>0))break}break k}for(;!((b=o)>>>0<=r>>>0|(n=i)>>>0<=k>>>0||(o=b-1|0,gf[0|(i=n-1|0)]!=gf[0|o])););for(i=t+1|0,15<=(o=n-k|0)>>>0?(df[0|t]=240,255<=(0|(a=o-15|0))&&(If(s=i,255,(i=(239+(n-(((0|a)<509?a:509)+k|0)|0)>>>0)/255|0)+1|0),a=(o+Bf(i,-255)|0)-270|0,i=2+(i+t|0)|0),df[0|i]=a,i=i+1|0):df[0|t]=o<<4,a=i+o|0;o=gf[k+4|0]|gf[k+5|0]<<8|(gf[k+6|0]<<16|gf[k+7|0]<<24),s=gf[0|k]|gf[k+1|0]<<8|(gf[k+2|0]<<16|gf[k+3|0]<<24),df[0|i]=s,df[i+1|0]=s>>>8,df[i+2|0]=s>>>16,df[i+3|0]=s>>>24,df[i+4|0]=o,df[i+5|0]=o>>>8,df[i+6|0]=o>>>16,df[i+7|0]=o>>>24,k=k+8|0,(i=i+8|0)>>>0<a>>>0;);for(k=n;;){i=k-b|0,df[0|a]=i,df[a+1|0]=i>>>8,i=b+4|0;o:{t:{if((n=c)>>>0<=(o=k+4|0)>>>0)b=o;else{if(i=(gf[0|o]|gf[o+1|0]<<8|(gf[o+2|0]<<16|gf[o+3|0]<<24))^(gf[0|i]|gf[i+1|0]<<8|(gf[i+2|0]<<16|gf[i+3|0]<<24)))break t;i=b+8|0,b=k+8|0}if(b>>>0<n>>>0)for(;;){if(n=(gf[0|b]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24))^(gf[0|i]|gf[i+1|0]<<8|(gf[i+2|0]<<16|gf[i+3|0]<<24))){i=((Rf(n)>>>3|0)+b|0)-o|0;break o}if(i=i+4|0,!((b=b+4|0)>>>0<c>>>0))break}(gf[0|i]|gf[i+1|0]<<8)!=(gf[0|b]|gf[b+1|0]<<8)|m>>>0<=b>>>0||(b=b+2|0,i=i+2|0),b>>>0<l>>>0&&(b=gf[0|i]==gf[0|b]?b+1|0:b),i=b-o|0;break o}i=Rf(i)>>>3|0}if(b=a+2|0,k=4+(i+k|0)|0,o=gf[0|t],15<=i>>>0?(df[0|t]=o+15,df[0|b]=255,df[b+1|0]=255,df[b+2|0]=255,df[b+3|0]=255,1020<=(o=i-15|0)>>>0&&(o=Bf(b=((i=i-1035|0)>>>0)/1020|0,-1020)+i|0,b=If(a+6|0,255,(i=b<<2)+4|0)+i|0),df[0|(a=b+(i=((65535&o)>>>0)/255|0)|0)]=i+o,b=a+1|0):df[0|t]=i+o,t=b,u>>>0<=k>>>0)break k;if(a=Bf(gf[0|(i=k-2|0)]|gf[i+1|0]<<8|(gf[i+2|0]<<16|gf[i+3|0]<<24),-1640531535)>>>20|0,d){if(wf[(a<<2)+f>>2]=i-h,a=(Bf(gf[0|k]|gf[k+1|0]<<8|(gf[k+2|0]<<16|gf[k+3|0]<<24),-1640531535)>>>18&16380)+f|0,i=wf[a>>2],b=a,a=k-h|0,wf[b>>2]=a,b=i+h|0,i+65535>>>0<a>>>0)continue b}else if(wf[(a<<2)+f>>2]=i,i=(Bf(gf[0|k]|gf[k+1|0]<<8|(gf[k+2|0]<<16|gf[k+3|0]<<24),-1640531535)>>>18&16380)+f|0,(b=wf[i>>2])+65535>>>0<(wf[i>>2]=k)>>>0)continue b;if((gf[0|b]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24))!=(gf[0|k]|gf[k+1|0]<<8|(gf[k+2|0]<<16|gf[k+3|0]<<24)))continue b;a=t+1|(df[0|t]=0)}}break}15<=(r=A-k|0)>>>0?(df[0|t]=240,f=t+1|0,(i=r-15|0)>>>0<255?df[0|(t=f)]=i:(If(a=f,255,(f=((i=r-270|0)>>>0)/255|0)+1|0),df[0|(t=2+(f+t|0)|0)]=i+Bf(f,-255))):df[0|t]=r<<4;break r}if(!(2113929216<i>>>0)){if(h=r+i|0,vf[f+16390>>1]=3,wf[f+16384>>2]=i,(0|(wf[f+16400>>2]=i))<13){b=e,k=r;break e}l=h-11|0,m=(A=h-5|(vf[(Bf(gf[0|r]|gf[r+1|0]<<8|(gf[r+2|0]<<16|gf[r+3|0]<<24),-1640531535)>>>18&16382)+f>>1]=0))-1|0,s=A-3|0,y=k<<6,b=e,k=r;a:for(;;){for(c=gf[o=k+1|0]|gf[k+2|0]<<8|(gf[k+3|0]<<16|gf[k+4|0]<<24),u=1,a=y;;){if(l>>>0<(o=u+(i=o)|0)>>>0)break e;if(t=Bf(c,-1640531535),c=gf[0|o]|gf[o+1|0]<<8|(gf[o+2|0]<<16|gf[o+3|0]<<24),n=Sf[(t=(t>>>18&16382)+f|0)>>1],vf[t>>1]=i-r,u=a>>6,a=a+1|0,(gf[0|(t=r+n|0)]|gf[t+1|0]<<8|(gf[t+2|0]<<16|gf[t+3|0]<<24))==(gf[0|i]|gf[i+1|0]<<8|(gf[i+2|0]<<16|gf[i+3|0]<<24)))break}for(;!((o=t)>>>0<=r>>>0|(n=i)>>>0<=k>>>0||(t=o-1|0,gf[0|(i=n-1|0)]!=gf[0|t])););for(i=b+1|0,15<=(a=n-k|0)>>>0?(df[0|b]=240,255<=(0|(c=a-15|0))&&(If(t=i,255,(i=(239+(n-(((0|c)<509?c:509)+k|0)|0)>>>0)/255|0)+1|0),c=(a+Bf(i,-255)|0)-270|0,i=2+(i+b|0)|0),df[0|i]=c,i=i+1|0):df[0|b]=a<<4,a=i+a|0;t=gf[k+4|0]|gf[k+5|0]<<8|(gf[k+6|0]<<16|gf[k+7|0]<<24),p=gf[0|k]|gf[k+1|0]<<8|(gf[k+2|0]<<16|gf[k+3|0]<<24),df[0|i]=p,df[i+1|0]=p>>>8,df[i+2|0]=p>>>16,df[i+3|0]=p>>>24,df[i+4|0]=t,df[i+5|0]=t>>>8,df[i+6|0]=t>>>16,df[i+7|0]=t>>>24,k=k+8|0,(i=i+8|0)>>>0<a>>>0;);for(k=n;;){n=b,i=k-o|0,df[0|a]=i,df[a+1|0]=i>>>8,i=o+4|0;b:{k:{if((p=s)>>>0<=(t=k+4|0)>>>0)b=t;else{if(i=(gf[0|t]|gf[t+1|0]<<8|(gf[t+2|0]<<16|gf[t+3|0]<<24))^(gf[0|i]|gf[i+1|0]<<8|(gf[i+2|0]<<16|gf[i+3|0]<<24)))break k;i=o+8|0,b=k+8|0}if(b>>>0<p>>>0)for(;;){if(o=(gf[0|b]|gf[b+1|0]<<8|(gf[b+2|0]<<16|gf[b+3|0]<<24))^(gf[0|i]|gf[i+1|0]<<8|(gf[i+2|0]<<16|gf[i+3|0]<<24))){i=((Rf(o)>>>3|0)+b|0)-t|0;break b}if(i=i+4|0,!((b=b+4|0)>>>0<s>>>0))break}(gf[0|i]|gf[i+1|0]<<8)!=(gf[0|b]|gf[b+1|0]<<8)|m>>>0<=b>>>0||(b=b+2|0,i=i+2|0),b>>>0<A>>>0&&(b=gf[0|i]==gf[0|b]?b+1|0:b),i=b-t|0;break b}i=Rf(i)>>>3|0}if(b=a+2|0,k=4+(i+k|0)|0,t=gf[0|n],15<=i>>>0?(df[0|n]=t+15,df[0|b]=255,df[b+1|0]=255,df[b+2|0]=255,df[b+3|0]=255,1020<=(o=i-15|0)>>>0&&(o=Bf(b=((i=i-1035|0)>>>0)/1020|0,-1020)+i|0,b=If(a+6|0,255,(i=b<<2)+4|0)+i|0),df[0|(a=b+(i=((65535&o)>>>0)/255|0)|0)]=i+o,b=a+1|0):df[0|n]=i+t,l>>>0<=k>>>0)break e;if(vf[(Bf(gf[0|(i=k-2|0)]|gf[i+1|0]<<8|(gf[i+2|0]<<16|gf[i+3|0]<<24),-1640531535)>>>18&16382)+f>>1]=i-r,i=(Bf(gf[0|k]|gf[k+1|0]<<8|(gf[k+2|0]<<16|gf[k+3|0]<<24),-1640531535)>>>18&16382)+f|0,a=Sf[i>>1],vf[i>>1]=k-r,(gf[0|(o=r+a|0)]|gf[o+1|0]<<8|(gf[o+2|0]<<16|gf[o+3|0]<<24))!=(gf[0|k]|gf[k+1|0]<<8|(gf[k+2|0]<<16|gf[k+3|0]<<24)))continue a;a=b+1|(df[0|b]=0)}}}}return d}15<=(r=h-k|0)>>>0?(df[0|b]=240,f=b+1|0,(i=r-15|0)>>>0<255?df[0|(b=f)]=i:(If(a=f,255,(f=((i=r-270|0)>>>0)/255|0)+1|0),df[0|(b=2+(f+b|0)|0)]=i+Bf(f,-255))):df[0|b]=r<<4;break f}return(Ef(t+1|0,k,r)+r|0)-e|0}return(Ef(b+1|0,k,r)+r|0)-e|0}(f,r,e,i,a,b)},function(f,r,e,i,a,b,k){return 0|n(f|=0,r|=0,e|=0,i|=0,a|=0,(0|(b|=0))<0?1-b|0:1)},function(f,r,e,i,a,b,k){return f|=0,r|=0,e|=0,i|=0,a|=0,k|=0,(0|(b|=0))<=2?k?(p(f),s(f,wf[k+4>>2])):s(f,0):(m(f,b),wf[f+262172>>2]=k?wf[k+8>>2]:0),k?0|v(f,r,e,i,a):0|(o=f,t=r,k=e,f=i,r=a,e=b,wf[12+(_f=a=_f-16|(b=a=i=0))>>2]=f,i=0,3&o||(i=gf[o+262171|0]?(vf[o+262170>>1]=0,wf[o+262172>>2]=0,-1):(wf[o+262172>>2]=0,wf[o+262144>>2]-wf[o+262148>>2]|0),b=(0|e)<1?9:e,vf[o+262168>>1]=(0|b)<12?b:12,1073741825<=i>>>0&&(If(If(o,0,131072)+131072|0,255,131072),i=0),wf[o+262144>>2]=t,b=i+65536|0,wf[o+262164>>2]=b,wf[o+262160>>2]=b,wf[o+262156>>2]=b,i=(t-i|0)-65536|0,wf[o+262148>>2]=i,wf[o+262152>>2]=i,i=((i=wf[o+262172>>2])?A:Zf)(o,t,k,12+a|0,r,e,(0|r)<(0|(f>>>0<=2113929216?16+(((f>>>0)/255|0)+f|0)|0:0))?1:0)),_f=16+a|0,i);var o,t},function(f,r,e,i,a,b,k){return 0|v(f|=0,r|=0,e|=0,i|=0,a|=0)}]).set=function(f,r){this[f]=r},f.get=function(f){return this[f]},f);return{e:function(){},f:function(){wf[506]=S(8192),wf[507]=S(8192)},g:function(f,r,e,i,a,b,k,o,t,n,s){return 0|h(f|=0,r|=0,e|=0,i|=0,a|=0,b|=0,k|=0,o|=0,t|=0,n|=0,s|=0)},h:function(f){var r=0;(r=wf[(f|=0)+4>>2])&&(B(wf[r+144>>2]),B(wf[r+76>>2]),B(r)),B(f)},i:function(f){var r=0,e=0,i=0;return _f=r=_f-32|0,4294967276<(e=b(wf[(f|=0)+4>>2],wf[507],wf[479],f+8|0))>>>0?(e=4294967277<=e>>>0?wf[1120-(e<<2)>>2]:1088,df[30+r|0]=0,df[28+r|0]=105,df[29+r|0]=105,wf[16+r>>2]=f,wf[20+r>>2]=e,o(2001,28+r|0,16+r|0)):(wf[r>>2]=f,wf[8+r>>2]=e,wf[4+r>>2]=wf[507],df[28+r|0]=105,df[29+r|0]=105,df[30+r|0]=105,df[31+r|0]=0,o(1975,28+r|0,0|r),i=1),_f=32+r|0,0|i},j:function(f){var r,e,i,a=0,b=0,k=0;return _f=a=_f-48|0,b=wf[(f|=0)+4>>2],wf[32+a>>2]=f,r=wf[506],wf[36+a>>2]=r,wf[40+a>>2]=8192,e=wf[479],i=wf[507],df[44+a|0]=105,df[45+a|0]=105,df[46+a|0]=105,4294967276<(b=function(f,r,e,i,a){var b,k,o,t,n=0,s=0,u=0,c=0,l=0,A=0,h=0,p=0,m=0;_f=m=_f-16|0,u=-1;f:if(1==wf[f+60>>2]&&(s=wf[f+36>>2],n=-2,c=l=wf[f+84>>2],p=e,4==(-4&(A=(A=wf[f>>2])||4))&&(n=wf[1200+(A<<2)>>2]),c=(l>>>0<(e=(A=n)-1|0)>>>0?c:e)+a|0,e=s|!a?e&c:0,u=-11,!(p>>>0<4+(((Bf(A,n=(c>>>0)/(n>>>0)|0)+(wf[f+8>>2]<<2)|0)+e|0)+Bf(n+(0!=(0|e))|0,4+(wf[f+28>>2]<<2)|0)|0)>>>0))){if(e=1==wf[f+4>>2],u=wf[f+32>>2]<3?e?1:2:e?3:4,A=i+a|0,c=wf[f+68>>2],wf[12+m>>2]=0,s=r,e=i,l&&(e=l+wf[f+80>>2]|0,a>>>0<(l=c-l|0)>>>0?(Ef(e,i,a),n=wf[f+84>>2]+a|0,e=A):(Ef(e,i,l),h=wf[f+28>>2],n=wf[f+80>>2],e=r+4|0,(s=0|F[0|u](wf[f+144>>2],n,e,c,c-1|0,wf[f+32>>2],wf[f+64>>2]))?(df[0|r]=s,df[r+1|0]=s>>>8,df[r+2|0]=s>>>16,df[r+3|0]=s>>>24):(df[0|r]=c,df[r+2|0]=c>>>16,df[r+1|0]=c>>>8,df[r+3|0]=c>>>24|128,Ef(e,n,c),s=c),h&&(n=e+s|0,e=Z(e,s),df[0|n]=e,df[n+1|0]=e>>>8,df[n+2|0]=e>>>16,df[n+3|0]=e>>>24),e=i+l|0,s=4+(((h<<2)+s|0)+r|0)|0,h=1,n=0,wf[f+4>>2]||(wf[f+80>>2]=c+wf[f+80>>2])),wf[f+84>>2]=n),c>>>0<=(n=A-e|0)>>>0)for(b=c>>>16|0,k=c>>>8|0,o=c-1|0,t=c>>>24|-128;p=wf[f+28>>2],l=s+4|0,(n=0|F[0|u](wf[f+144>>2],e,l,c,o,wf[f+32>>2],wf[f+64>>2]))?(df[0|s]=n,df[s+1|0]=n>>>8,df[s+2|0]=n>>>16,df[s+3|0]=n>>>24):(df[s+3|0]=t,df[s+2|0]=b,df[s+1|0]=k,Ef(l,e,df[0|s]=c),n=c),p&&(h=n+l|0,l=Z(l,n),df[0|h]=l,df[h+1|0]=l>>>8,df[h+2|0]=l>>>16,df[h+3|0]=l>>>24),s=4+(((p<<(h=2))+n|0)+s|0)|0,c>>>0<=(n=A-(e=e+c|0)|0)>>>0;);if(!wf[f+36>>2]|A>>>0<=e>>>0||(p=wf[f+28>>2],l=s+4|0,(u=0|F[0|u](wf[f+144>>2],e,l,n,n-1|0,wf[f+32>>2],wf[f+64>>2]))?(df[0|s]=u,df[s+1|0]=u>>>8,df[s+2|0]=u>>>16,df[s+3|0]=u>>>24,n=u):(df[0|s]=n,df[s+2|0]=n>>>16,df[s+1|0]=n>>>8,df[s+3|0]=n>>>24|128,Ef(l,e,n)),p&&(e=n+l|0,u=Z(l,n),df[0|e]=u,df[e+1|0]=u>>>8,df[e+2|0]=u>>>16,df[e+3|0]=u>>>24),s=4+(((p<<(h=2))+n|0)+s|0)|0,e=A),wf[f+4>>2]||2!=(0|h))u=wf[f+76>>2],n=wf[f+80>>2];else if(wf[12+m>>2])u=wf[f+76>>2],n=wf[f+80>>2]=u;else{if(u=-1,!(n=(wf[f+32>>2]<=2?w:y)(wf[f+144>>2],wf[f+76>>2])))break f;n=n+(u=wf[f+76>>2])|0,wf[f+80>>2]=n}wf[f+36>>2]|n+c>>>0<=wf[f+72>>2]+u>>>0||(n=(n=(wf[(c=f)+32>>2]<=2?w:y)(wf[f+144>>2],u))+wf[f+76>>2]|0,wf[c+80>>2]=n),e>>>0<A>>>0&&(Ef(n,c=e,e=A-e|0),wf[f+84>>2]=e),1==wf[f+8>>2]&&_(f+96|0,i,a),i=wf[(e=f)+88>>2]+a|0,f=wf[f+92>>2],wf[e+88>>2]=i,wf[e+92>>2]=i>>>0<a>>>0?f+1|0:f,u=s-r|0}return _f=16+m|0,u}(b,i,e,r,(df[47+a|0]=0)|o(1943,44+a|0,32+a|0)))>>>0?(b=4294967277<=b>>>0?wf[1120-(b<<2)>>2]:1088,df[46+a|0]=0,df[44+a|0]=105,df[45+a|0]=105,wf[16+a>>2]=f,wf[20+a>>2]=b,o(2001,44+a|0,16+a|0)):(wf[a>>2]=f,wf[8+a>>2]=b,wf[4+a>>2]=wf[507],df[44+a|0]=105,df[45+a|0]=105,df[46+a|0]=105,df[47+a|0]=0,o(1975,44+a|0,0|a),k=1),_f=48+a|0,0|k},k:function(f){var r=0,e=0,i=0;return _f=r=_f-32|0,4294967276<(e=c(wf[(f|=0)+4>>2],wf[507],wf[479]))>>>0?(e=4294967277<=e>>>0?wf[1120-(e<<2)>>2]:1088,df[30+r|0]=0,df[28+r|0]=105,df[29+r|0]=105,wf[16+r>>2]=f,wf[20+r>>2]=e,o(2001,28+r|0,16+r|0)):(wf[r>>2]=f,wf[8+r>>2]=e,wf[4+r>>2]=wf[507],df[28+r|0]=105,df[29+r|0]=105,df[30+r|0]=105,df[31+r|0]=0,o(1975,28+r|0,0|r),i=1),_f=32+r|0,0|i},l:function(){var f,r,e=0,i=S(4);return f=i,r=0,4294967276<((r=J(208))?(wf[r+32>>2]=100,wf[f>>2]=r,0):(wf[f>>2]=0,-9))>>>0||(e=i,8191<g[479]||(B(wf[507]),wf[479]=8192,wf[507]=S(8192))),0|e},m:function(f){(f=wf[(f|=0)>>2])&&(B(wf[f+56>>2]),B(wf[f+68>>2]),B(f))},n:function(f){f|=0;var r,e,i,a=0,b=0,k=0;for(wf[32+(_f=a=_f+-64|0)>>2]=f,wf[40+a>>2]=8192,wf[36+a>>2]=wf[506],df[60+a|0]=105,df[61+a|0]=105,df[62+a|0]=105,e=(df[63+a|0]=0)|o(1943,60+a|0,32+a|0),wf[56+a>>2]=8192;;){if(wf[52+a>>2]=e-b,4294967276<(r=function(f,r,e,i,a){var b,k,o,t,n,s=0,u=0,c=0,l=0,A=0,h=0,p=0,m=0,y=0,d=0,v=0,w=0;_f=w=_f-16|0,l=wf[a>>2],s=wf[e>>2],wf[12+w>>2]=0,wf[a>>2]=0,t=f+140|(wf[e>>2]=0),d=f+188|0,k=f+92|0,n=f+192|0,o=r+s|0,b=i+l|0,l=i,A=r,y=1;f:{for(;;){r:{e:{i:{a:{b:{k:{o:{t:{n:{s:{u:{c:{l:{A:{h:{p:{m:{y:{d:{v:{w:{g:{S:{B:{_:{Z:{E:switch(wf[f+36>>2]){case 1:s=b-l|0,c=wf[f+64>>2],u=wf[f+60>>2];break Z;case 9:s=o-A|0,c=wf[f+84>>2],u=wf[f+88>>2];break h;case 13:break n;case 12:break s;case 11:break l;case 10:break A;case 8:break m;case 7:break y;case 6:break d;case 5:break v;case 4:break S;case 3:break B;case 2:break _;case 0:break E;case 14:break k;default:continue}if(19<=(s=b-l|0)>>>0){if(4294967277<=(u=E(f,l,s))>>>0)break f;l=u+l|0;continue}if(wf[f+60>>2]=0,!s){u=7;break f}wf[f+36>>2]=1,c=wf[f+64>>2]=7,u=0}if(Ef(h=188+(u+f|0)|0,l,s=(u=c-u|0)>>>0<s>>>0?u:s),u=s+wf[f+60>>2]|0,wf[f+60>>2]=u,l=s+l|0,(s=wf[f+64>>2])>>>0<=u>>>0)break b;y=4+(s-u|0)|0;break r}if(wf[f+8>>2]&&R(k),(c=wf[f+48>>2]+(!wf[f+4>>2]<<17)|0)>>>0<=g[f+52>>2])s=wf[f+68>>2];else{if(wf[f+52>>2]=0,B(wf[f+56>>2]),s=S(wf[f+48>>2]+4|0),u=-9,!(wf[f+56>>2]=s))break f;if(B(wf[f+68>>2]),s=S(c),!(wf[f+68>>2]=s))break f;wf[f+52>>2]=c}wf[f+80>>2]=s,wf[f+60>>2]=0,wf[f+64>>2]=0,wf[f+84>>2]=0,wf[f+88>>2]=0,wf[f+36>>2]=3}if((s=b-l|0)>>>0<=3){wf[f+36>>2]=4,wf[f+60>>2]=0;break g}l=(s=l)+4|0;break w}s=b-l|0}if(s=(c=4-(u=wf[f+60>>2])|0)>>>0<s>>>0?c:s,Ef(u+wf[f+56>>2]|0,l,s),u=s+wf[f+60>>2]|0,l=s+l|0,(wf[f+60>>2]=u)>>>0<=3){y=4-u|0;break r}s=wf[f+56>>2]}if(!(s=2147483647&(c=gf[0|s]|gf[s+1|0]<<8|(gf[s+2|0]<<16|gf[s+3|0]<<24)))){wf[f+36>>2]=10;continue}if(s>>>0>g[f+48>>2]){u=-2;break f}if(u=wf[f+28>>2],(0|c)<=-1){wf[f+64>>2]=s,u&&R(t),wf[f+36>>2]=5;continue}if(wf[f+36>>2]=7,s=s+(u<<2)|0,u=(wf[f+64>>2]=s)+4|0,y=(s=(0|A)==(0|o)|(0|l)==(0|b))?u:y,1^s)continue;break r}if(s=Ef(h=A,l,c=(s=wf[f+64>>2])>>>0<(A=(u=b-l|0)>>>0<(A=o-A|0)>>>0?u:A)>>>0?s:A),wf[f+28>>2]&&_(t,l,c),wf[f+8>>2]&&_(k,l,c),wf[f+16>>2]|wf[f+20>>2]&&(u=wf[f+40>>2],h=wf[f+44>>2]-(u>>>0<c>>>0)|0,wf[f+40>>2]=u-c,wf[f+44>>2]=h),wf[f+4>>2]||(h=s,u=wf[f+76>>2],A=wf[f+72>>2],u||(A=wf[f+72>>2]=s),(0|(m=h))!=(0|(h=u+A|0))?65536<=(p=c+(s-r|0)|0)>>>0?wf[f+72>>2]=r:p=(0|(m=wf[f+68>>2]))!=(0|A)?(Ef(m,h-(A=u>>>0<(A=65536-c|0)>>>0?u:A)|0,A),Ef(A+wf[f+68>>2]|0,s,c),wf[f+72>>2]=wf[f+68>>2],c+A|0):(g[f+52>>2]<u+c>>>0&&(Ef(A,(c+h|0)-65536|0,u=65536-c|0),wf[f+76>>2]=u,A=wf[f+68>>2]),Ef(u+A|0,s,c),c+wf[f+76>>2]|0):p=u+c|0,wf[f+76>>2]=p),A=s+c|0,l=c+l|0,(0|(s=wf[f+64>>2]))==(0|c)){u=wf[(s=f)+28>>2]?(wf[f+60>>2]=0,6):3,wf[s+36>>2]=u;continue}s=s-c|0,y=4+((wf[f+64>>2]=s)+((0!=wf[f+28>>2])<<2)|0)|0;break r}if(!((s=wf[f+60>>2])|(0|(u=b-l|0))<4)){s=l+4|0;break e}if(Ef(h=188+(f+s|0)|0,l,s=(s=4-s|0)>>>0<u>>>0?s:u),u=s+wf[f+60>>2]|0,s=s+l|0,l=d,4<=(wf[f+60>>2]=u)>>>0)break e;l=s;break r}if(b-l>>>0<(s=wf[f+64>>2])>>>0){wf[f+36>>2]=8,wf[f+60>>2]=0;continue}l=(u=l)+s|0;break p}if(u=wf[f+60>>2],s=(s=wf[f+64>>2]-u|0)>>>0<(c=b-l|0)>>>0?s:c,Ef(u+wf[f+56>>2]|0,l,s),u=s+wf[f+60>>2]|0,l=s+l|0,(wf[f+60>>2]=u)>>>0<(s=wf[f+64>>2])>>>0){y=4+((s-u|0)+((0!=wf[f+28>>2])<<2)|0)|0;break r}u=wf[f+56>>2]}if(wf[f+28>>2]&&(c=s-4|0,wf[f+64>>2]=c,(gf[0|(s=u+c|0)]|gf[s+1|0]<<8|(gf[s+2|0]<<16|gf[s+3|0]<<24))!=(0|Z(u,c)))){u=-7;break f}if(s=o-A|0,(p=wf[f+48>>2])>>>0<=s>>>0){if(h=0!=(0|(s=wf[f+72>>2]))&1073741824<(c=wf[f+76>>2])>>>0,(0|(c=U(u,A,wf[f+64>>2],p,h?(s+c|0)-65536|0:s,h?65536:c)))<0){u=-1;break f}wf[f+8>>2]&&_(k,A,c),wf[f+16>>2]|wf[f+20>>2]&&(u=wf[f+40>>2],s=c,h=wf[f+44>>2]-(u>>>0<s>>>0)|0,wf[f+40>>2]=u-s,wf[f+44>>2]=h),wf[f+4>>2]||(h=A,u=wf[f+76>>2],s=wf[f+72>>2],u||(s=wf[f+72>>2]=A),(0|(m=h))!=(0|(h=s+u|0))?65536<=(p=c+(A-r|0)|0)>>>0?wf[f+72>>2]=r:p=(0|(m=wf[f+68>>2]))!=(0|s)?(Ef(m,h-(s=u>>>0<(s=65536-c|0)>>>0?u:s)|0,s),Ef(s+wf[f+68>>2]|0,A,c),wf[f+72>>2]=wf[f+68>>2],s+c|0):(g[f+52>>2]<u+c>>>0&&(Ef(s,(c+h|0)-65536|0,u=65536-c|0),wf[f+76>>2]=u,s=wf[f+68>>2]),Ef(s+u|0,A,c),c+wf[f+76>>2]|0):p=u+c|0,wf[f+76>>2]=p),wf[f+36>>2]=3,A=c+A|0;continue}if(wf[f+4>>2]?(h=wf[f+80>>2],c=wf[f+76>>2]):(c=wf[f+76>>2],h=(0|(h=wf[(m=f)+72>>2]))!=(0|(v=wf[f+68>>2]))?(c>>>0<65536?c:65536)+v|0:(131073<=c>>>0&&(Ef(h,(c+h|0)-65536|0,65536),wf[f+76>>2]=65536,p=wf[f+48>>2],h=wf[f+68>>2],c=65536),c+h|0),wf[m+80>>2]=h),v=0!=(0|(m=wf[f+72>>2]))&1073741824<c>>>0,(0|(c=U(u,h,wf[f+64>>2],p,v?(c+m|0)-65536|0:m,v?65536:c)))<0){u=-16;break f}wf[f+8>>2]&&_(k,wf[f+80>>2],c),wf[f+16>>2]|wf[f+20>>2]&&(h=wf[f+40>>2],m=wf[f+44>>2]-(h>>>0<c>>>0)|0,wf[f+40>>2]=h-c,wf[f+44>>2]=m),wf[f+88>>2]=0,wf[f+84>>2]=c,wf[f+36>>2]=9,u=0}if(h=A,c=(A=c-u|0)>>>0<s>>>0?A:s,s=Ef(h,u+wf[f+80>>2]|0,c),wf[f+4>>2]||(u=s,h=wf[f+76>>2],A=wf[f+72>>2],h||(A=wf[f+72>>2]=s),(0|(m=A+h|0))!=(0|u)?65536<=(p=c+(s-r|0)|0)>>>0?wf[f+72>>2]=r:p=(0|(u=A))!=(0|(A=wf[f+68>>2]))?(Ef((h=wf[f+80>>2])-(A=(u=h-A|0)>>>0<(A=65536<(A=65536-wf[f+84>>2]|0)>>>0?0:A)>>>0?u:A)|0,(m-wf[f+88>>2]|0)-A|0,A),wf[f+72>>2]=wf[f+68>>2],wf[f+88>>2]+(u+c|0)|0):c+h|0:p=c+h|0,wf[f+76>>2]=p),u=c+wf[f+88>>2]|0,A=s+c|0,(0|(wf[f+88>>2]=u))!=wf[f+84>>2]){y=4;break r}wf[f+36>>2]=3;continue}if(u=-14,wf[f+40>>2]|wf[f+44>>2])break f;if(!wf[f+8>>2])break i;if((0|(s=b-l|0))<=3){wf[f+36>>2]=11,wf[f+60>>2]=0;break c}s=l+4|0;break u}s=b-l|0}if(s=(c=4-(u=wf[f+60>>2])|0)>>>0<s>>>0?c:s,Ef(u+wf[f+56>>2]|0,l,s),u=s+wf[f+60>>2]|0,s=s+l|0,(wf[f+60>>2]=u)>>>0<=3){y=4-u|0,l=s;break r}l=wf[f+56>>2]}if(u=-18,(gf[0|l]|gf[l+1|0]<<8|(gf[l+2|0]<<16|gf[l+3|0]<<24))!=(0|C(k)))break f;wf[f+72>>2]=0,y=wf[f+76>>2]=0,wf[f+36>>2]=0,l=s;break r}if((0|(s=b-l|0))<=3){wf[f+60>>2]=4,wf[f+64>>2]=8,wf[f+36>>2]=13;break t}l=(s=l)+4|0;break o}s=b-l|0}if(Ef(188+(f+(u=wf[f+60>>2])|0)|0,l,s=(c=wf[f+64>>2]-u|0)>>>0<s>>>0?c:s),u=s+wf[f+60>>2]|0,l=s+l|0,(wf[f+60>>2]=u)>>>0<(s=wf[f+64>>2])>>>0)break a;s=n}s=gf[0|s]|gf[s+1|0]<<8|(gf[s+2|0]<<16|gf[s+3|0]<<24),wf[f+64>>2]=s,wf[f+36>>2]=14,wf[f+16>>2]=s,wf[f+20>>2]=0;continue}if(y=(s=wf[f+64>>2])-(u=s>>>0<(u=b-l|0)>>>0?s:u)|0,l=u+l|0,wf[f+64>>2]=y)break r;break i}if(4294967277<=(u=E(f,d,s))>>>0)break f;continue}y=s-u|0;break r}wf[f+72>>2]=0,y=wf[f+76>>2]=0,wf[f+36>>2]=0;break r}if((gf[0|l]|gf[l+1|0]<<8|(gf[l+2|0]<<16|gf[l+3|0]<<24))!=(0|C(t))){u=-7;break f}wf[f+36>>2]=3,l=s;continue}break}wf[f+4>>2]||(s=wf[f+68>>2],d=wf[f+72>>2],wf[12+w>>2]|(0|s)==(0|d)||7<(u=wf[f+36>>2])-2>>>0||(9!=(0|u)?((u=(c=wf[f+76>>2])>>>0<65536?c:65536)&&(Ef(s,(c+d|0)-u|0,u),s=wf[f+68>>2]),wf[f+76>>2]=u,wf[f+72>>2]=s,wf[f+80>>2]=s+u):((h=(c=wf[(u=f)+80>>2]-s|0)>>>0<(h=65536<(h=65536-wf[f+84>>2]|0)>>>0?0:h)>>>0?c:h)&&(Ef((s+c|0)-h|0,((d+wf[f+76>>2]|0)-wf[f+88>>2]|0)-h|0,h),s=wf[f+68>>2]),wf[u+72>>2]=s,wf[f+76>>2]=c+wf[f+88>>2]))),wf[a>>2]=l-i,wf[e>>2]=A-r,u=y}return _f=16+w|0,u}(wf[f>>2],wf[507],56+a|0,wf[506]+b|0,52+a|0))>>>0)b=4294967277<=r>>>(k=0)?wf[1120-(r<<2)>>2]:1088,df[62+a|0]=0,df[60+a|0]=105,df[61+a|0]=105,wf[16+a>>2]=f,wf[20+a>>2]=b,o(2001,60+a|0,16+a|0);else if(i=wf[52+a>>2],k=wf[56+a>>2],k&&(wf[a>>2]=f,wf[8+a>>2]=k,wf[4+a>>2]=wf[507],df[60+a|0]=105,df[61+a|0]=105,df[62+a|0]=105,df[63+a|0]=0,o(1975,60+a|0,0|a)),k=1,r&&(b=b+i|0,8192==wf[56+a>>2]|b>>>0<e>>>0))continue;break}return _f=a+64|0,0|k},o:function(f,r){return r|=0,0|(df[15+(_f=r=_f-16|(r=0))|0]=0,o(1924,15+r|0,0),_f=16+r|0,0)},p:F}}(f)}(Y)}var d=Error,v=[];"object"!=typeof{}&&O("no native wasm support detected");var w,g,S,B=!1,_="undefined"!=typeof TextDecoder?new TextDecoder("utf8"):void 0,Z=i.INITIAL_MEMORY||33554432,u=i.wasmMemory||new function(){this.buffer=new ArrayBuffer(Z/65536*65536)};u&&(E=u.buffer),Z=E.byteLength;var c=E,E=c;i.HEAP8=new Int8Array(c),i.HEAP16=new Int16Array(c),i.HEAP32=g=new Int32Array(c),i.HEAPU8=w=new Uint8Array(c),i.HEAPU16=new Uint16Array(c),i.HEAPU32=new Uint32Array(c),i.HEAPF32=new Float32Array(c),i.HEAPF64=S=new Float64Array(c);var I,C=[],R=[],U=[],J=[];var F=0,M=null,x=null;function O(f){throw i.onAbort&&i.onAbort(f),h(f),B=!0,f=new d("abort("+f+"). Build with -s ASSERTIONS=1 for more info."),a(f),f}function V(f){var r=W;return String.prototype.startsWith?f.startsWith(r):0===f.indexOf(r)}i.preloadedImages={},i.preloadedAudios={};var W="data:application/octet-stream;base64,",L="_lz4.wasm";V(L)||(c=L,L=i.locateFile?i.locateFile(c,A):A+c);var j={1924:function(){G()},1943:function(f,r,e){return tf(f,r)},1975:function(f,r,e){nf(f,r,e)},2001:function(f,r){sf(f,r)}};function T(f){for(;0<f.length;){var r,e=f.shift();"function"==typeof e?e(i):"number"==typeof(r=e.S)?void 0===e.F?I.get(r)():I.get(r)(e.F):r(void 0===e.F?null:e.F)}}var N=[],X=!1,D="function"==typeof atob?atob:function(f){var r="",e=0;f=f.replace(/[^A-Za-z0-9\+\/=]/g,"");do{var i="ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=".indexOf(f.charAt(e++)),a="ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=".indexOf(f.charAt(e++)),b="ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=".indexOf(f.charAt(e++)),k="ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=".indexOf(f.charAt(e++)),i=i<<2|a>>4,a=(15&a)<<4|b>>2,o=(3&b)<<6|k}while(r+=String.fromCharCode(i),64!==b&&(r+=String.fromCharCode(a)),64!==k&&(r+=String.fromCharCode(o)),e<f.length);return r};function P(r){if(V(r)){if(r=r.slice(W.length),"boolean"==typeof l&&l){try{var e=Buffer.from(r,"base64")}catch(f){e=new Buffer(r,"base64")}var f=new Uint8Array(e.buffer,e.byteOffset,e.byteLength)}else try{for(var i=D(r),a=new Uint8Array(i.length),e=0;e<i.length;++e)a[e]=i.charCodeAt(e);f=a}catch(f){throw Error("Converting base64 string to bytes failed.")}return f}}var Y={b:function(f,r,e){var i;for(N.length=0,e>>=2;i=w[r++];)(i=i<105)&&1&e&&e++,N.push(i?S[e++>>1]:g[e]),++e;return j[f].apply(null,N)},c:function(f,r,e){w.copyWithin(f,r,r+e)},d:function(){O("OOM")},a:u},u=function(){function f(f){i.asm=f.exports,I=i.asm.p,R.unshift(i.asm.e),F--,i.monitorRunDependencies&&i.monitorRunDependencies(F),0==F&&(null!==M&&(clearInterval(M),M=null),x&&(f=x,x=null,f()))}var r={a:Y};if(F++,i.monitorRunDependencies&&i.monitorRunDependencies(F),i.instantiateWasm)try{return i.instantiateWasm(r,f)}catch(f){return h("Module.instantiateWasm callback failed with error: "+f),!1}return f((r=function(){var r=L;try{try{if(r==L&&v)new Uint8Array(v);else if(!P(r)){if(!t)throw"sync fetching of the wasm failed: you can preload it to Module['wasmBinary'] manually, or emcc.py will do that for you when generating HTML (but not JS)";t(r)}}catch(f){O(f)}var f=new m,e=new y}catch(f){throw r=f.toString(),h("failed to compile wasm module: "+r),(0<=r.indexOf("imported Memory")||0<=r.indexOf("memory import"))&&h("Memory size incompatibility issues may be due to changing INITIAL_MEMORY at runtime to something too large. Use ALLOW_MEMORY_GROWTH to allow any size memory (and also make sure not to set INITIAL_MEMORY at runtime to something smaller than it was at compile time)."),f}return[e,f]}())[0]),i.asm}();i.___wasm_call_ctors=u.e;var z,G=i._LZ4JS_init=u.f,Q=i._LZ4JS_createCompressionContext=u.g,H=i._LZ4JS_freeCompressionContext=u.h,q=i._LZ4JS_compressBegin=u.i,K=i._LZ4JS_compressUpdate=u.j,$=i._LZ4JS_compressEnd=u.k,ff=i._LZ4JS_createDecompressionContext=u.l,rf=i._LZ4JS_freeDecompressionContext=u.m,ef=i._LZ4JS_decompress=u.n;function af(f){this.name="ExitStatus",this.message="Program terminated with exit("+f+")",this.status=f}function bf(){function f(){if(!z&&(z=!0,i.calledRun=!0,!B)){if(T(R),T(U),e(i),i.onRuntimeInitialized&&i.onRuntimeInitialized(),kf){var r=i._main;try{var f=r(0,0);p&&0===f||(p||(i.onExit&&i.onExit(f),B=!0),k(f,new af(f)))}catch(f){f instanceof af||"unwind"==f||((r=f)&&"object"==typeof f&&f.stack&&(r=[f,f.stack]),h("exception thrown: "+r),k(1,f))}}if(i.postRun)for("function"==typeof i.postRun&&(i.postRun=[i.postRun]);i.postRun.length;)r=i.postRun.shift(),J.unshift(r);T(J)}}if(!(0<F)){if(i.preRun)for("function"==typeof i.preRun&&(i.preRun=[i.preRun]);i.preRun.length;)r=void 0,r=i.preRun.shift(),C.unshift(r);T(C),0<F||(i.setStatus?(i.setStatus("Running..."),setTimeout(function(){setTimeout(function(){i.setStatus("")},1),f()},1)):f())}var r}if(i._main=u.o,x=function f(){z||bf(),z||(x=f)},i.run=bf,i.preInit)for("function"==typeof i.preInit&&(i.preInit=[i.preInit]);0<i.preInit.length;)i.preInit.pop()();var kf=!0;i.noInitialRun&&(kf=!1),bf();var of={};function tf(f,r){return of[f].H(r)}function nf(f,r,e){return of[f].I(r,e)}function sf(f,r){f=of[f];var e=Error;if(r){for(var i=r+NaN,a=r;w[a]&&!(i<=a);)++a;if(16<a-r&&w.subarray&&_)r=_.decode(w.subarray(r,a));else{for(i="";r<a;){var b,k,o=w[r++];128&o?(b=63&w[r++],192==(224&o)?i+=String.fromCharCode((31&o)<<6|b):(k=63&w[r++],(o=224==(240&o)?(15&o)<<12|b<<6|k:(7&o)<<18|b<<12|k<<6|63&w[r++])<65536?i+=String.fromCharCode(o):(o-=65536,i+=String.fromCharCode(55296|o>>10,56320|1023&o)))):i+=String.fromCharCode(o)}r=i}}else r="";f.B=e(r)}var uf={"64KB":4,"256KB":5,"1MB":6,"4MB":7};function cf(f){void 0===f&&(f={}),this.O={blockSizeID:uf["4MB"],blockMode:0,contentChecksumFlag:0,frameType:0,dictID:0,blockChecksumFlag:0},this.P={compressionLevel:0,autoFlush:1,favorDecSpeed:1},this.options={},this.options.frameInfo=Object.assign({},this.O,f.frameInfo),this.options.preferences=Object.assign({},this.options.frameInfo,this.P,f.preferences),this.B=null}function lf(f,r){if(f.v=Q(f.options.frameInfo.blockSizeID,f.options.frameInfo.blockMode,f.options.frameInfo.contentChecksumFlag,f.options.frameInfo.frameType,r||0,f.options.frameInfo.dictID,f.options.frameInfo.blockChecksumFlag,f.options.preferences.compressionLevel,f.options.preferences.autoFlush,f.options.preferences.favorDecSpeed),!f.v)throw Error("LZ4JS_createCompressionContext");of[f.v]=f}function Af(){}function hf(f){var r=0,e=f.reduce(function(f,r){return f+r.length},0),i=new Uint8Array(e);return f.forEach(function(f){i.set(f,r),r+=f.length}),i}function pf(f){function r(r,e){Object.getOwnPropertyNames(e).concat(Object.getOwnPropertySymbols(e)).forEach(function(f){f.match(/^(?:initializer|constructor|prototype|arguments|caller|name|bind|call|apply|toString|length)$/)||Object.defineProperty(r,f,Object.getOwnPropertyDescriptor(e,f))})}for(var e=[],i=arguments.length-1;0<i--;)e[i]=arguments[i+1];var a,b=((a=f)&&(k.__proto__=a),(k.prototype=Object.create(a&&a.prototype)).constructor=k);function k(){for(var r=[],f=arguments.length;f--;)r[f]=arguments[f];a.apply(this,r),e.forEach(function(f){"function"==typeof f.prototype.R&&f.prototype.R.apply(this,r)})}return e.forEach(function(f){r(b.prototype,f.prototype),r(b,f)}),b}cf.prototype.u=function(){if(H(this.v),delete of[this.v],this.B)throw this.B},Af.prototype.I=function(f,r){this.D.push(new Uint8Array(w.subarray(f,f+r)))},Af.prototype.H=function(f){return w.set(this.src.subarray(this.offset,this.offset+this.s),f),this.s};var mf,yf=((mf=pf(cf,Af))&&(df.__proto__=mf),((df.prototype=Object.create(mf&&mf.prototype)).constructor=df).prototype.L=function(){for(;this.offset<this.src.length;this.offset+=8192)this.s=Math.min(this.src.length-this.offset,8192),K(this.v)||this.u()},df.prototype.K=function(f){this.src=f,lf(this,f.length),q(this.v)||this.u(),this.L(),$(this.v),this.u();var r=hf(this.D),r=l&&Buffer.isBuffer(f)?Buffer.from(r.buffer,r.byteOffset,r.byteOffset+r.length):r;return this.C(),r},df.prototype.C=function(){this.src=null,this.s=this.offset=0,this.D=[]},df);function df(f){mf.call(this,f),this.C()}function vf(){if(this.A=ff(),!this.A)throw Error("LZ4JS_createDecompressionContext");of[this.A]=this}vf.prototype.u=function(){if(rf(this.A),delete of[this.A],this.B)throw this.B};var wf,gf,Sf,Bf,_f=((wf=pf(vf,Af))&&(Zf.__proto__=wf),((Zf.prototype=Object.create(wf&&wf.prototype)).constructor=Zf).prototype.N=function(){for(;this.offset<this.src.length;this.offset+=8192)this.s=Math.min(this.src.length-this.offset,8192),ef(this.A)||this.u();this.u()},Zf.prototype.M=function(f){this.src=f,this.N();var r=hf(this.D);return f=l&&Buffer.isBuffer(f)?Buffer.from(r.buffer):r,this.C(),f},Zf.prototype.C=function(){this.src=null,this.offset=0,this.D=[],this.s=0},Zf);function Zf(){wf.call(this),this.C()}function Ef(){}Ef.prototype.H=function(f){return w.set(new Uint8Array(this.src.buffer,this.src.byteOffset,this.s),f),this.s},Ef.prototype.I=function(f,r){this.G=Buffer.from(w.buffer).slice(f,f+r),this.push(Buffer.from(this.G))};try{gf=require("stream").Transform,(Bf=pf(gf,cf,Ef))&&(If.__proto__=Bf),((If.prototype=Object.create(Bf&&Bf.prototype)).constructor=If).prototype._transform=function(f,r,e){try{var i;for(this.J||(q(this.v)||this.u(),this.J=!0),i=0;i<f.length;i+=8192)this.s=Math.min(f.length-i,8192),this.src=f.slice(i,i+this.s),K(this.v)||this.u();e()}catch(f){e(f)}},If.prototype._flush=function(r){try{$(this.v),this.u(),r()}catch(f){r(f)}},Sf=If}catch(f){}function If(f){Bf.call(this,f),cf.call(this,f),this.J=!1,this.s=0,this.src=Buffer.alloc(0),this.G=Buffer.alloc(0),lf(this,0)}var Cf,Rf,Uf,Jf=Sf;try{Cf=require("stream").Transform,(Uf=pf(Cf,vf,Ef))&&(Ff.__proto__=Uf),((Ff.prototype=Object.create(Uf&&Uf.prototype)).constructor=Ff).prototype._transform=function(f,r,e){try{for(var i=0;i<f.length;i+=8192)this.s=Math.min(f.length-i,8192),this.src=f.slice(i,i+this.s),ef(this.A)||this.u();e()}catch(f){e(f)}},Ff.prototype._flush=function(f){this.u(),f()},Rf=Ff}catch(f){}function Ff(f){Uf.call(this,f),vf.call(this),this.s=0,this.src=Buffer.alloc(0),this.G=Buffer.alloc(0)}var Mf=Rf,u={BLOCK_MAX_SIZE:uf,LZ4JS_instances:of,LZ4JS_read:tf,LZ4JS_write:nf,LZ4JS_error:sf,compress:function(f,r){return new yf(r).K(f)},decompress:function(f){return(new _f).M(f)}};return l&&(u.createCompressStream=function(f){return new Jf(f)},u.createDecompressStream=function(){return new Mf}),i.BLOCK_MAX_SIZE=uf,i.lz4js=u,f}}();"object"==typeof exports&&"object"==typeof module?module.exports=lz4init:"function"==typeof define&&define.amd?define([],function(){return lz4init}):"object"==typeof exports&&(exports.lz4init=lz4init);



var CRC32;!function(n){"undefined"==typeof DO_NOT_EXPORT_CRC?"object"==typeof exports?n(exports):"function"==typeof define&&define.amd?define(function(){var r={};return n(r),r}):n(CRC32={}):n(CRC32={})}(function(r){r.version="1.2.0";var a=function(){for(var r=0,n=new Array(256),e=0;256!=e;++e)r=1&(r=1&(r=1&(r=1&(r=1&(r=1&(r=1&(r=1&(r=e)?-306674912^r>>>1:r>>>1)?-306674912^r>>>1:r>>>1)?-306674912^r>>>1:r>>>1)?-306674912^r>>>1:r>>>1)?-306674912^r>>>1:r>>>1)?-306674912^r>>>1:r>>>1)?-306674912^r>>>1:r>>>1)?-306674912^r>>>1:r>>>1,n[e]=r;return"undefined"!=typeof Int32Array?new Int32Array(n):n}();r.table=a,r.bstr=function(r,n){for(var e=-1^n,t=r.length-1,o=0;o<t;)e=(e=e>>>8^a[255&(e^r.charCodeAt(o++))])>>>8^a[255&(e^r.charCodeAt(o++))];return o===t&&(e=e>>>8^a[255&(e^r.charCodeAt(o))]),-1^e},r.buf=function(r,n){if(1e4<r.length)return function(r,n){for(var e=-1^n,t=r.length-7,o=0;o<t;)e=(e=(e=(e=(e=(e=(e=(e=e>>>8^a[255&(e^r[o++])])>>>8^a[255&(e^r[o++])])>>>8^a[255&(e^r[o++])])>>>8^a[255&(e^r[o++])])>>>8^a[255&(e^r[o++])])>>>8^a[255&(e^r[o++])])>>>8^a[255&(e^r[o++])])>>>8^a[255&(e^r[o++])];for(;o<7+t;)e=e>>>8^a[255&(e^r[o++])];return-1^e}(r,n);for(var e=-1^n,t=r.length-3,o=0;o<t;)e=(e=(e=(e=e>>>8^a[255&(e^r[o++])])>>>8^a[255&(e^r[o++])])>>>8^a[255&(e^r[o++])])>>>8^a[255&(e^r[o++])];for(;o<3+t;)e=e>>>8^a[255&(e^r[o++])];return-1^e},r.str=function(r,n){for(var e,t,o=-1^n,f=0,u=r.length;f<u;)o=(e=r.charCodeAt(f++))<128?o>>>8^a[255&(o^e)]:e<2048?(o=o>>>8^a[255&(o^(192|e>>6&31))])>>>8^a[255&(o^(128|63&e))]:55296<=e&&e<57344?(e=64+(1023&e),t=1023&r.charCodeAt(f++),(o=(o=(o=o>>>8^a[255&(o^(240|e>>8&7))])>>>8^a[255&(o^(128|e>>2&63))])>>>8^a[255&(o^(128|t>>6&15|(3&e)<<4))])>>>8^a[255&(o^(128|63&t))]):(o=(o=o>>>8^a[255&(o^(224|e>>12&15))])>>>8^a[255&(o^(128|e>>6&63))])>>>8^a[255&(o^(128|63&e))];return-1^o}});

/*
Copyright (c) 2011, Daniel Guerrero
All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:
    * Redistributions of source code must retain the above copyright
      notice, this list of conditions and the following disclaimer.
    * Redistributions in binary form must reproduce the above copyright
      notice, this list of conditions and the following disclaimer in the
      documentation and/or other materials provided with the distribution.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL DANIEL GUERRERO BE LIABLE FOR ANY
DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
(INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
(INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

/**
 * Uses the new array typed in javascript to binary base64 encode/decode
 * at the moment just decodes a binary base64 encoded
 * into either an ArrayBuffer (decodeArrayBuffer)
 * or into an Uint8Array (decode)
 *
 * References:
 * https://developer.mozilla.org/en/JavaScript_typed_arrays/ArrayBuffer
 * https://developer.mozilla.org/en/JavaScript_typed_arrays/Uint8Array
 */

var Base64Binary={_keyStr:"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=",decodeArrayBuffer:function(r){var e=r.length/4*3,e=new ArrayBuffer(e);return this.decode(r,e),e},removePaddingChars:function(r){return 64==this._keyStr.indexOf(r.charAt(r.length-1))?r.substring(0,r.length-1):r},decode:function(r,e){r=this.removePaddingChars(r),r=this.removePaddingChars(r);var t,n,a,i,h,d=parseInt(r.length/4*3,10),s=0,f=0,c=e?new Uint8Array(e):new Uint8Array(d);for(r=r.replace(/[^A-Za-z0-9\+\/\=]/g,""),s=0;s<d;s+=3)t=this._keyStr.indexOf(r.charAt(f++))<<2|(a=this._keyStr.indexOf(r.charAt(f++)))>>4,n=(15&a)<<4|(i=this._keyStr.indexOf(r.charAt(f++)))>>2,a=(3&i)<<6|(h=this._keyStr.indexOf(r.charAt(f++))),c[s]=t,64!=i&&(c[s+1]=n),64!=h&&(c[s+2]=a);return c}};

//The WASM binary is stored in base64
const wasmModuleAsArrayBuffer = Base64Binary.decodeArrayBuffer(String.raw`AGFzbQEAAAABbA5gAX8Bf2AHf39/f39/fwF/YAN/f38Bf2ABfwBgBn9/f39/fwF/YAABf2ACf38B
f2AFf39/f38Bf2AEf39/fwF/YAAAYAJ/fwBgA39/fwBgC39/f39/f39/f39/AX9gCn9/f39+f39/
f38BfwITAwFhAWEAAgFhAWIAAgFhAWMAAAM7OgICAwACAAYLAQoAAAMCBgoBBgMJBwIEBAMFAAwA
AAQFAwQHAwAAAAINAQEBAQIHCAcECQgIBQQGAAUEBQFwAQUFBQYBAYAEgAQGCQF/AUHwk8ACCwc1
DQFkAgABZQAWAWYANQFnAB4BaAAjAWkAIAFqAB8BawAdAWwAHAFtABsBbgA7AW8AOgFwAQAJCgEA
QQELBC8uLSwKq8MFOvICAgJ/AX4CQCACRQ0AIAAgAmoiA0EBayABOgAAIAAgAToAACACQQNJDQAg
A0ECayABOgAAIAAgAToAASADQQNrIAE6AAAgACABOgACIAJBB0kNACADQQRrIAE6AAAgACABOgAD
IAJBCUkNACAAQQAgAGtBA3EiBGoiAyABQf8BcUGBgoQIbCIBNgIAIAMgAiAEa0F8cSIEaiICQQRr
IAE2AgAgBEEJSQ0AIAMgATYCCCADIAE2AgQgAkEIayABNgIAIAJBDGsgATYCACAEQRlJDQAgAyAB
NgIYIAMgATYCFCADIAE2AhAgAyABNgIMIAJBEGsgATYCACACQRRrIAE2AgAgAkEYayABNgIAIAJB
HGsgATYCACAEIANBBHFBGHIiBGsiAkEgSQ0AIAGtQoGAgIAQfiEFIAMgBGohAQNAIAEgBTcDGCAB
IAU3AxAgASAFNwMIIAEgBTcDACABQSBqIQEgAkEgayICQR9LDQALCyAAC4IEAQN/IAJBgARPBEAg
ACABIAIQARogAA8LIAAgAmohAwJAIAAgAXNBA3FFBEACQCACQQFIBEAgACECDAELIABBA3FFBEAg
ACECDAELIAAhAgNAIAIgAS0AADoAACABQQFqIQEgAkEBaiICIANPDQEgAkEDcQ0ACwsCQCADQXxx
IgRBwABJDQAgAiAEQUBqIgVLDQADQCACIAEoAgA2AgAgAiABKAIENgIEIAIgASgCCDYCCCACIAEo
Agw2AgwgAiABKAIQNgIQIAIgASgCFDYCFCACIAEoAhg2AhggAiABKAIcNgIcIAIgASgCIDYCICAC
IAEoAiQ2AiQgAiABKAIoNgIoIAIgASgCLDYCLCACIAEoAjA2AjAgAiABKAI0NgI0IAIgASgCODYC
OCACIAEoAjw2AjwgAUFAayEBIAJBQGsiAiAFTQ0ACwsgAiAETw0BA0AgAiABKAIANgIAIAFBBGoh
ASACQQRqIgIgBEkNAAsMAQsgA0EESQRAIAAhAgwBCyAAIANBBGsiBEsEQCAAIQIMAQsgACECA0Ag
AiABLQAAOgAAIAIgAS0AAToAASACIAEtAAI6AAIgAiABLQADOgADIAFBBGohASACQQRqIgIgBE0N
AAsLIAIgA0kEQANAIAIgAS0AADoAACABQQFqIQEgAkEBaiICIANHDQALCyAAC6cMAQd/AkAgAEUN
ACAAQQhrIgMgAEEEaygCACIBQXhxIgBqIQUCQCABQQFxDQAgAUEDcUUNASADIAMoAgAiAWsiA0GE
ECgCAEkNASAAIAFqIQAgA0GIECgCAEcEQCABQf8BTQRAIAMoAggiAiABQQN2IgRBA3RBnBBqRhog
AiADKAIMIgFGBEBB9A9B9A8oAgBBfiAEd3E2AgAMAwsgAiABNgIMIAEgAjYCCAwCCyADKAIYIQYC
QCADIAMoAgwiAUcEQCADKAIIIgIgATYCDCABIAI2AggMAQsCQCADQRRqIgIoAgAiBA0AIANBEGoi
AigCACIEDQBBACEBDAELA0AgAiEHIAQiAUEUaiICKAIAIgQNACABQRBqIQIgASgCECIEDQALIAdB
ADYCAAsgBkUNAQJAIAMgAygCHCICQQJ0QaQSaiIEKAIARgRAIAQgATYCACABDQFB+A9B+A8oAgBB
fiACd3E2AgAMAwsgBkEQQRQgBigCECADRhtqIAE2AgAgAUUNAgsgASAGNgIYIAMoAhAiAgRAIAEg
AjYCECACIAE2AhgLIAMoAhQiAkUNASABIAI2AhQgAiABNgIYDAELIAUoAgQiAUEDcUEDRw0AQfwP
IAA2AgAgBSABQX5xNgIEIAMgAEEBcjYCBCAAIANqIAA2AgAPCyADIAVPDQAgBSgCBCIBQQFxRQ0A
AkAgAUECcUUEQCAFQYwQKAIARgRAQYwQIAM2AgBBgBBBgBAoAgAgAGoiADYCACADIABBAXI2AgQg
A0GIECgCAEcNA0H8D0EANgIAQYgQQQA2AgAPCyAFQYgQKAIARgRAQYgQIAM2AgBB/A9B/A8oAgAg
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
HXYhASAAQQF0IQAgAiABQQRxaiIDKAIQIgENAAsgAyAHNgIQIAcgAjYCGAsgByAHNgIMIAcgBzYC
CAwDCyADKAIIIgAgBjYCDCADIAY2AgggBkEANgIYIAYgAzYCDCAGIAA2AggLIAlBCGohAAwFCyAC
KAIIIgAgBzYCDCACIAc2AgggB0EANgIYIAcgAjYCDCAHIAA2AggLQYAQKAIAIgAgCE0NAEGAECAA
IAhrIgE2AgBBjBBBjBAoAgAiAiAIaiIANgIAIAAgAUEBcjYCBCACIAhBA3I2AgQgAkEIaiEADAML
QfAPQTA2AgBBACEADAILAkAgBUUNAAJAIAQoAhwiAkECdEGkEmoiACgCACAERgRAIAAgATYCACAB
DQFB+A8gCUF+IAJ3cSIJNgIADAILIAVBEEEUIAUoAhAgBEYbaiABNgIAIAFFDQELIAEgBTYCGCAE
KAIQIgAEQCABIAA2AhAgACABNgIYCyAEKAIUIgBFDQAgASAANgIUIAAgATYCGAsCQCADQQ9NBEAg
BCADIAhqIgBBA3I2AgQgACAEaiIAIAAoAgRBAXI2AgQMAQsgBCAIQQNyNgIEIAYgA0EBcjYCBCAD
IAZqIAM2AgAgA0H/AU0EQCADQQN2IgBBA3RBnBBqIQICf0H0DygCACIBQQEgAHQiAHFFBEBB9A8g
ACABcjYCACACDAELIAIoAggLIQAgAiAGNgIIIAAgBjYCDCAGIAI2AgwgBiAANgIIDAELQR8hACAD
Qf///wdNBEAgA0EIdiIAIABBgP4/akEQdkEIcSICdCIAIABBgOAfakEQdkEEcSIBdCIAIABBgIAP
akEQdkECcSIAdEEPdiABIAJyIAByayIAQQF0IAMgAEEVanZBAXFyQRxqIQALIAYgADYCHCAGQgA3
AhAgAEECdEGkEmohAgJAAkAgCUEBIAB0IgFxRQRAQfgPIAEgCXI2AgAgAiAGNgIAIAYgAjYCGAwB
CyADQQBBGSAAQQF2ayAAQR9GG3QhACACKAIAIQgDQCAIIgEoAgRBeHEgA0YNAiAAQR12IQIgAEEB
dCEAIAEgAkEEcWoiAigCECIIDQALIAIgBjYCECAGIAE2AhgLIAYgBjYCDCAGIAY2AggMAQsgASgC
CCIAIAY2AgwgASAGNgIIIAZBADYCGCAGIAE2AgwgBiAANgIICyAEQQhqIQAMAQsCQCALRQ0AAkAg
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
QbwBaiAHIAggBmsiBiAFIAUgBksbIgUQBBogACAFIAAoAjxqIgY2AjwgBSAHaiEHIAAoAkAiBSAG
TQ0UIAUgBmtBBGohDQwYCyAAKAIIBEAgDxAPCwJAIAAoAjAgACgCBEVBEXRqIgggACgCNE0EQCAA
KAJEIQUMAQsgAEEANgI0IAAoAjgQBSAAIAAoAjBBBGoQBiIFNgI4QXchBiAFRQ0aIAAoAkQQBSAA
IAgQBiIFNgJEIAVFDRogACAINgI0CyAAIAU2AlAgAEIANwI8IABCADcCVCAAQQM2AiQLIAwgB2si
BUEDTQRAIABBBDYCJCAAQQA2AjwMAgsgByIFQQRqIQcMAgsgDCAHayEFCyAAKAI8IgYgACgCOGog
B0EEIAZrIgYgBSAFIAZLGyIFEAQaIAAgACgCPCAFaiIGNgI8IAUgB2ohByAGQQNNBEBBBCAGayEN
DBULIAAoAjghBQsgBSgAACIIQf////8HcSIFRQRAIABBCjYCJAwVCyAAKAIwIAVJBEBBfiEGDBYL
IAAoAhwhBiAIQX9MBEAgACAFNgJAIAYEQCATEA8LIABBBTYCJAwVCyAAQQc2AiQgACAFIAZBAnRq
IgU2AkAgBUEEaiANIAkgEUYgByAMRnIiBRshDSAFQQFzDRQMEwsgCSAHIAAoAkAiBSAMIAdrIgYg
ESAJayIJIAYgCUkbIgYgBSAGSRsiBRAEIQkgACgCHARAIBMgByAFEAoLIAAoAggEQCAPIAcgBRAK
CyAAKQMQUEUEQCAAIAApAyggBa19NwMoCyAAKAIERQRAAkAgCQJ/IAAoAkwiBgRAIAAoAkgMAQsg
ACAJNgJIIAkLIgggBmoiC0YEQCAFIAZqIQoMAQsgBSAJIAFraiIKQYCABE8EQCAAIAE2AkgMAQsg
ACgCRCIKIAhGBEAgACgCNCAFIAZqSQR/IAggBSALakGAgARrQYCABCAFayIGEAQaIAAgBjYCTCAA
KAJEBSAICyAGaiAJIAUQBBogACgCTCAFaiEKDAELIAogCyAGQYCABCAFayIIIAYgCEkbIgZrIAYQ
BBogACgCRCAGaiAJIAUQBBogACAAKAJENgJIIAUgBmohCgsgACAKNgJMCyAFIAlqIQkgBSAHaiEH
IAUgACgCQCIGRgRAIAACf0EDIAAoAhxFDQAaIABBADYCPEEGCzYCJAwUCyAAIAYgBWsiBTYCQCAF
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
AEcNASAFQQA6AAAgBUEBaiEEDAALAAsACyALDwsCQCARIAZrIgFBD08EQCAFQfABOgAAIAVBAWoh
ACABQQ9rIgNB/wFJBEAgACIFIAM6AAAMAgsgAEH/ASABQY4CayIDQf8BbiIAQQFqEAMaIAAgBWpB
AmoiBSAAQYF+bCADajoAAAwBCyAFIAFBBHQ6AAALIAVBAWogBiABEAQgAWogAmsPCyAIQQFqIAYg
ARAEIAFqIAJrCwQAEDwLvQIBBn8jAEFAaiIBJAAgASAANgIgIAFBgMAANgIoIAFB6A8oAgA2AiQg
AUHp0qUDNgA8QZcPIAFBPGogAUEgahAAIQUgAUGAwAA2AjgDQAJAIAEgBSAEazYCNCAAKAIAQewP
KAIAIAFBOGpB6A8oAgAgBGogAUE0ahAlIgJBbEsEQEEAIQMgAkFtTwR/QeAIIAJBAnRrKAIABUHA
CAshAiABQQA6AD4gAUHp0gE7ADwgASAANgIQIAEgAjYCFEHRDyABQTxqIAFBEGoQABoMAQsgASgC
NCEGIAEoAjgiAwRAIAEgADYCACABIAM2AgggAUHsDygCADYCBCABQenSpQM2ADxBtw8gAUE8aiAB
EAAaC0EBIQMgAkUNACAFIAQgBmoiBEsNASABKAI4QYDAAEYNAQsLIAFBQGskACADCyoBAX8jAEEQ
ayIAJAAgAEEAOgAPQYQPIABBD2pBABAAGiAAQRBqJABBAAsL7AYGAEGECAsZAQAAAAIAAAABAAAA
AAAAAAQAAAAEAAAABABBrAgLKv/////8////AQAAAAIAAAADAAAAVW5zcGVjaWZpZWQgZXJyb3Ig
Y29kZQBB4AgLUtAEAADbBAAA6QQAAAQFAAAcBQAAPgUAAF0FAAB3BQAAkwUAAKoFAADCBQAA2QUA
APMFAAAQBgAAKAYAAD4GAABRBgAAawYAAIgGAACmBgAAyQYAQcIJC5QEAQAAAAQAAAAQAAAAQABP
S19Ob0Vycm9yAEVSUk9SX0dFTkVSSUMARVJST1JfbWF4QmxvY2tTaXplX2ludmFsaWQARVJST1Jf
YmxvY2tNb2RlX2ludmFsaWQARVJST1JfY29udGVudENoZWNrc3VtRmxhZ19pbnZhbGlkAEVSUk9S
X2NvbXByZXNzaW9uTGV2ZWxfaW52YWxpZABFUlJPUl9oZWFkZXJWZXJzaW9uX3dyb25nAEVSUk9S
X2Jsb2NrQ2hlY2tzdW1faW52YWxpZABFUlJPUl9yZXNlcnZlZEZsYWdfc2V0AEVSUk9SX2FsbG9j
YXRpb25fZmFpbGVkAEVSUk9SX3NyY1NpemVfdG9vTGFyZ2UARVJST1JfZHN0TWF4U2l6ZV90b29T
bWFsbABFUlJPUl9mcmFtZUhlYWRlcl9pbmNvbXBsZXRlAEVSUk9SX2ZyYW1lVHlwZV91bmtub3du
AEVSUk9SX2ZyYW1lU2l6ZV93cm9uZwBFUlJPUl9zcmNQdHJfd3JvbmcARVJST1JfZGVjb21wcmVz
c2lvbkZhaWxlZABFUlJPUl9oZWFkZXJDaGVja3N1bV9pbnZhbGlkAEVSUk9SX2NvbnRlbnRDaGVj
a3N1bV9pbnZhbGlkAEVSUk9SX2ZyYW1lRGVjb2RpbmdfYWxyZWFkeVN0YXJ0ZWQARVJST1JfbWF4
Q29kZQBB5A0LlgECAAAAEAAAAAAAAAACAAAAEAAAAAAAAAACAAAAEAAAAAAAAAAEAAAAEAAAAAAA
AAAIAAAAEAAAAAAAAAAQAAAAEAAAAAAAAAAgAAAAEAAAAAAAAABAAAAAEAAAAAAAAACAAAAAEAAA
AAAAAAAAAQAAEAAAAAEAAABgAAAAQAAAAAEAAAAAAgAAgAAAAAEAAAAAQAAAABAAQf0OCwYgAADw
CVA=
`);

const lz4 = lz4init({ wasmBinary: wasmModuleAsArrayBuffer });

var customMapFormat = {
    name: "Custom map format",
    extension: "custom",

    write: function(map, fileName) {
        let array = [];;

        array.push((map.width * 15 & 0xff000000) >> 24);
        array.push((map.width * 15 & 0x00ff0000) >> 16);
        array.push((map.width * 15 & 0x0000ff00) >> 8);
        array.push((map.width * 15 & 0x000000ff));

        array.push((map.height * 15 & 0xff000000) >> 24);
        array.push((map.height * 15 & 0x00ff0000) >> 16);
        array.push((map.height * 15 & 0x0000ff00) >> 8);
        array.push((map.height * 15 & 0x000000ff));

        for (i = map.layerCount  - 1; i >= 0; i--) {
            var layer = map.layerAt(i);

            if (layer.isTileLayer) {
                for (y = 0; y < layer.height; ++y) {
                    for (x = 0; x < layer.width; ++x)
                        if (layer.tileAt(x, y) != null) {
                            // x coor
                            array.push((x * 15 & 0xff000000) >> 24);
                            array.push((x * 15 & 0x00ff0000) >> 16);
                            array.push((x * 15 & 0x0000ff00) >> 8);
                            array.push((x * 15 & 0x000000ff));

                            //y coor
                            array.push((y * 15 & 0xff000000) >> 24);
                            array.push((y * 15 & 0x00ff0000) >> 16);
                            array.push((y * 15 & 0x0000ff00) >> 8);
                            array.push(y * 15 & 0x000000ff);

                            //width
                            array.push((15 & 0xff000000) >> 24);
                            array.push((15 & 0x00ff0000) >> 16);
                            array.push((15 & 0x0000ff00) >> 8);
                            array.push(15 & 0x000000ff);

                            //height
                            array.push((15 & 0xff000000) >> 24);
                            array.push((15 & 0x00ff0000) >> 16);
                            array.push((15 & 0x0000ff00) >> 8);
                           array.push(15 & 0x000000ff);

                            if (layer.tileAt(x, y).property("player_spawn") == true) {
                                array.push(255);

                            } else {
                                array.push(0);

                            }

                            if (layer.tileAt(x, y).property("player_collidable") == true) {
                                array.push(255);

                            } else {
                                array.push(0);

                            }

                            array.push(layer.tileAt(x, y).property("red"));
                            array.push(layer.tileAt(x, y).property("green"));
                            array.push(layer.tileAt(x, y).property("blue"));
                            array.push(layer.tileAt(x, y).property("alpha"));
                            array.push(0);


                            for (var z = 0; z < array.length; z += 23) {
                                var is_null = true;

                                for (var j = 0; j <= 22; j ++) {
                                    if (array[j + z] != 0) {
                                        is_null = false;
                                        break;

                                    }
                                }

                                if (is_null == true) {
                                    index += 23;
                                    break;

                                }
                            }




                        }

                }

            }
        }

        // An entirely null map object signifies the start of the crc32 hash
        for (var i = 1; i < 22; i ++) {
            array.push(0);

        }

        //array.push(1);


        var file = new BinaryFile(fileName, BinaryFile.WriteOnly);

        let tmp_byte_array = new Uint8Array(array);

	const lz4 = lz4init().lz4js;

        let crc32 = CRC32.buf(tmp_byte_array);

        array.push((crc32 & 0xff000000) >> 24);
        array.push((crc32 & 0x00ff0000) >> 16);
        array.push((crc32 & 0x0000ff00) >> 8);
        array.push(crc32 & 0x000000ff);

        let byte_array = new Uint8Array(array);

        //Compress it
        let compressed_byte_array = lz4.compress(byte_array, {
                preferences: {
                    compressionLevel: 0
                }
            });

        file.write(compressed_byte_array.buffer);
        file.commit();
    },
}

tiled.registerMapFormat("custom", customMapFormat)
