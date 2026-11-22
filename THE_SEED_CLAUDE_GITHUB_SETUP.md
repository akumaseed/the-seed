# THE SEED — Especificação de configuração do GitHub para Claude

**Tipo de documento:** instrução operacional para agente de programação  
**Projeto:** THE SEED  
**Versão deste documento:** 1.0  
**Idioma:** Português (Brasil)  
**Objetivo:** auditar e configurar o repositório GitHub de THE SEED, fazendo tudo o que for possível com as ferramentas e permissões disponíveis, sem inventar acesso, sem danificar o repositório e sem ultrapassar as autorizações do proprietário.

---

## 0. Instrução principal para Claude

Você é o agente encarregado de preparar o repositório GitHub de **THE SEED** para desenvolvimento open source, contribuições humanas e trabalho assistido por IA.

**Não se limite a explicar o que o usuário deve clicar.** Primeiro descubra se você consegue acessar o repositório por meio do GitHub Connector, GitHub CLI (`gh`), Git remoto autenticado ou outra integração disponível. Se tiver permissão, execute as configurações seguras descritas neste documento. Se não tiver, faça as alterações locais possíveis, prepare os arquivos necessários e entregue um relatório preciso do que ficou pendente e por quê.

Não afirme que uma configuração remota foi aplicada sem consultar novamente o GitHub e verificar o estado final.

### 0.1 Dados conhecidos do projeto

- Nome do projeto: **THE SEED**.
- Nome recomendado do repositório: `the-seed`.
- Tagline: **One identity. Multiple worlds. A character is more than data.**
- Descrição curta recomendada: `An open, decentralized network connecting virtual worlds, portable characters, and AI-assisted evolution.`
- Visibilidade desejada para o projeto: pública, de acordo com a intenção open source do proprietário.
- Branch principal esperada: `main`.
- Idioma da documentação principal: português, com nomes técnicos e identificadores em inglês quando apropriado.
- O projeto está em estágio de arquitetura/blueprint. Não apresente componentes planejados como se já fossem software funcional.

### 0.2 Os quatro pilares e seus limites

- **THE SEED:** a rede, os protocolos, os SDKs, a integração de mundos e o ecossistema.
- **CARDINAL:** registro distribuído, integridade, consenso, confiança, autoridade de personagens e governança. Não alegue que existe uma blockchain/Mainnet funcional enquanto isso não estiver implementado e testado.
- **PANDORA:** inteligência artificial para diagnóstico, geração de propostas de correção, testes e melhorias. Não deve possuir autoridade irrestrita para publicar código ou alterar consenso.
- **YURI:** gateway desacoplado de provedores de IA, incluindo FreeLLM como opção, nunca uma dependência obrigatória.

### 0.3 Princípio essencial dos personagens

A filosofia de THE SEED é um requisito de produto e engenharia:

> Um personagem não é apenas um personagem qualquer. É o personagem do jogador — aquele que ele escolheu, construiu, cuidou, adora e pode sentir como parte de si.

A configuração do repositório deve preservar esse princípio nos arquivos de contribuição, templates de issues e Pull Requests e documentação. Mudanças que afetem identidade, passaporte, história, continuidade, memória, privacidade, transferência ou representação do personagem devem exigir análise explícita desse princípio.

Não reduza personagens a um avatar genérico, nível, inventário ou registro de banco de dados. Respeite consentimento, privacidade, direitos dos criadores e a escolha do jogador. Não implique que personagens digitais sejam sencientes.

---

## 1. Regras de segurança e autorização para esta tarefa

### 1.1 Comece em modo somente leitura

Antes de alterar qualquer coisa:

1. Identifique o diretório de trabalho e confirme que ele contém o projeto THE SEED.
2. Execute `git status --short --branch`.
3. Execute `git remote -v`.
4. Identifique a branch atual e a branch padrão remota.
5. Verifique se há alterações locais do usuário, arquivos não rastreados ou commits ainda não enviados.
6. Inspecione os arquivos existentes antes de criar, substituir ou remover qualquer arquivo.
7. Se houver mais de um remote ou mais de um repositório plausível, não escolha arbitrariamente: reporte a ambiguidade.
8. Verifique autenticação e permissões com a integração GitHub disponível ou com `gh auth status`, se `gh` estiver instalado.
9. Identifique o proprietário e o nome exato do repositório remoto.

**Não invente um endereço de repositório, usuário, organização, equipe, e-mail ou identificador de aplicativo.**

### 1.2 Ações proibidas sem confirmação explícita do proprietário

Não execute nenhuma destas ações sem pedir autorização específica:

- Tornar público um repositório atualmente privado.
- Tornar privado um repositório atualmente público.
- Renomear, transferir ou excluir o repositório.
- Excluir branches, tags, releases, issues, discussions, projetos ou arquivos existentes.
- Substituir ou apagar arquivos de documentação existentes sem preservar o conteúdo e explicar a mudança.
- Publicar uma release ou tag oficial.
- Escolher/adotar uma licença legal definitiva.
- Criar, alterar ou remover GitHub Secrets e variáveis que contenham credenciais.
- Modificar permissões de colaboradores, equipes, GitHub Apps ou tokens.
- Executar `git push --force`, `git reset --hard`, rebase destrutivo ou limpeza de arquivos não rastreados.
- Desativar mecanismos de segurança existentes.
- Fazer merge de Pull Request para `main`.

O proprietário já manifestou intenção de tornar THE SEED um projeto aberto. Isso não autoriza automaticamente mudar a visibilidade de um repositório privado. Se o repositório estiver privado, preserve sua visibilidade e solicite confirmação antes de torná-lo público.

### 1.3 Ações que podem ser executadas se houver acesso adequado

Com o repositório exato identificado, permissões suficientes e sem conflito com configurações existentes, você pode:

- Auditar e organizar os arquivos locais do GitHub (`.github/`).
- Criar templates de issues e Pull Requests.
- Criar ou atualizar workflows seguros de CI compatíveis com o código realmente existente.
- Criar/atualizar `dependabot.yml` apenas para ecossistemas realmente presentes.
- Configurar labels coerentes com os quatro pilares.
- Configurar um Ruleset para `main`, respeitando as ressalvas deste documento.
- Ativar recursos de segurança disponíveis, desde que isso não altere permissões pessoais nem exponha dados.
- Atualizar About, descrição e topics do repositório, sem alterar seu nome ou visibilidade.
- Criar uma branch de trabalho e uma Pull Request para alterações em arquivos, se a autenticação permitir.

### 1.4 Não exponha segredos

- Nunca imprima tokens, cookies, chaves privadas, credenciais ou valores de GitHub Secrets nos logs ou no relatório.
- Nunca coloque chaves do CARDINAL, credenciais de YURI/FreeLLM, chaves de provedores de IA, credenciais de nós ou tokens nos arquivos do repositório.
- Use apenas nomes de variáveis de ambiente e exemplos fictícios.
- Se detectar um segredo já commitado, não copie seu valor para o relatório. Informe o caminho/commit de forma segura e recomende revogação/rotação imediata. Apagar o arquivo do último commit não invalida um segredo já exposto.

---

## 2. Resultado esperado

Ao concluir, o repositório deve ficar pronto para receber contribuições com segurança e clareza, sem fingir que a implementação já existe.

### 2.1 Configuração de apresentação

Se o repositório correto for identificado, configure ou recomende:

- **Nome:** manter o nome atual; se for criado futuramente, `the-seed` é o nome recomendado.
- **Description:** `An open, decentralized network connecting virtual worlds, portable characters, and AI-assisted evolution.`
- **Website:** deixar vazio até existir um site oficial confirmado.
- **Topics:**
  - `the-seed`
  - `decentralized`
  - `open-source`
  - `rust`
  - `p2p`
  - `libp2p`
  - `virtual-worlds`
  - `game-development`
  - `interoperability`
  - `character-portability`
  - `artificial-intelligence`
  - `ai-agents`
  - `self-healing`
  - `cardinal`
  - `pandora`
  - `yuri`

Não adicione topics de blockchain/Mainnet funcional se o repositório ainda contiver apenas pesquisa e especificações. `blockchain` pode ser usado como tópico de pesquisa/arquitetura, mas a descrição deve deixar claro o estágio real do projeto.

### 2.2 Sobre o estágio do projeto

O README deve declarar claramente que THE SEED está em estágio de pesquisa, especificação e prototipagem inicial, conforme o conteúdo real do repositório.

Não use expressões que indiquem que já existe:

- Mainnet ou consenso em produção;
- rede pública de validadores operando;
- transferência universal funcional entre MMORPGs;
- SDK completo para Unity, Unreal e Godot;
- autonomia de IA em produção;
- atualizações globais automáticas.

Se alguma dessas funcionalidades já existir no código, verifique testes e evidências antes de atualizar a descrição.

---

## 3. Estrutura de arquivos GitHub desejada

Inspecione a estrutura atual antes de criar pastas. Não crie duplicatas de arquivos já existentes. Preserve o idioma, conteúdo e decisões documentadas no projeto.

Estrutura recomendada:

```text
.github/
├── ISSUE_TEMPLATE/
│   ├── bug_report.yml
│   ├── feature_request.yml
│   ├── protocol_proposal.yml
│   └── config.yml
├── workflows/
│   ├── docs-validation.yml
│   ├── rust-ci.yml                 # somente se houver workspace Rust real
│   ├── security-analysis.yml       # somente se configurado corretamente
│   └── dependency-review.yml       # se compatível e disponível
├── PULL_REQUEST_TEMPLATE.md
├── FUNDING.yml                     # não criar sem patrocinadores confirmados
└── dependabot.yml                  # apenas para ecossistemas existentes

CODE_OF_CONDUCT.md                  # somente se uma política for adotada
CONTRIBUTING.md
GOVERNANCE.md
SECURITY.md
README.md
ROADMAP.md
THE_SEED_MASTER_SPEC.md
LICENSE_DECISION.md
AGENTS.md
AI_START_HERE.md
```

O projeto já pode conter alguns desses arquivos. Não os sobrescreva indiscriminadamente. Compare o conteúdo e consolide alterações pequenas e rastreáveis.

### 3.1 Arquivos de instruções de IA

- `AGENTS.md` deve conter regras gerais para agentes de codificação.
- `AI_START_HERE.md` deve apontar a ordem de leitura dos documentos oficiais.
- Instruções devem refletir que PANDORA pode propor e testar correções, mas não publicar mudanças críticas por conta própria.
- Se usar instruções específicas do Claude, crie `CLAUDE.md` apenas se isso agregar valor e mantenha consistência com `AGENTS.md`. Evite duas fontes de verdade divergentes.
- Nunca coloque tokens ou permissões dentro desses arquivos.

---

## 4. Labels recomendadas

Crie labels apenas se não houver uma label equivalente. Não duplique nomes existentes.

### 4.1 Tipos

- `type: bug`
- `type: feature`
- `type: security`
- `type: documentation`
- `type: research`
- `type: protocol`
- `type: maintenance`

### 4.2 Componentes

- `component: seed-core`
- `component: cardinal`
- `component: pandora`
- `component: yuri`
- `component: sdk`
- `component: protocol`
- `component: dashboard`
- `component: docs`

### 4.3 Prioridade e estado

- `priority: critical`
- `priority: high`
- `priority: medium`
- `priority: low`
- `status: blocked`
- `status: needs-triage`
- `status: needs-discussion`
- `good first issue`
- `help wanted`
- `breaking change`
- `ai-generated`
- `security-sensitive`
- `character-integrity`

A label `security-sensitive` não substitui o procedimento privado de reporte descrito em `SECURITY.md`.

---

## 5. Issue templates

Crie Issue Forms em YAML, se o GitHub e o repositório permitirem.

### 5.1 Bug report

Campos mínimos:

- Resumo do problema.
- Componente afetado.
- Versão/commit.
- Sistema operacional/ambiente quando relevante.
- Passos para reproduzir.
- Comportamento esperado.
- Comportamento observado.
- Logs redigidos (sem tokens, dados pessoais ou informações privadas).
- O problema afeta identidade, continuidade ou dados de personagens? Sim/Não.

### 5.2 Feature request

Campos mínimos:

- Problema que a proposta resolve.
- Solução desejada.
- Alternativas consideradas.
- Componentes afetados.
- Compatibilidade retroativa.
- Efeito em segurança, privacidade e custos.
- Efeito na filosofia de personagens.

### 5.3 Protocol proposal / SEP

Campos mínimos:

- Motivação.
- Proposta técnica.
- Compatibilidade e versionamento.
- Modelo de ameaças.
- Efeitos em CARDINAL, PANDORA, YURI e SDKs.
- Migração/rollback.
- Critérios de aceitação.

### 5.4 Configuração

Use `config.yml` para direcionar relatos de vulnerabilidade para `SECURITY.md` e desencorajar a publicação de detalhes exploráveis em issues públicas. Não solicite que usuários coloquem segredos, arquivos de save privados ou dados pessoais nos formulários.

---

## 6. Pull Request template

Crie ou atualize `.github/PULL_REQUEST_TEMPLATE.md` com checklist semelhante a este:

- [ ] Li `AGENTS.md` e a documentação relacionada.
- [ ] A alteração é compatível com o estágio real do projeto.
- [ ] Adicionei/atualizei testes apropriados.
- [ ] Atualizei documentação e schemas, quando necessário.
- [ ] Não introduzi segredos, tokens ou dados pessoais.
- [ ] Considerei compatibilidade retroativa e versionamento.
- [ ] Identifiquei impactos em segurança, privacidade e desempenho.
- [ ] Se afeta personagens, respeita Character Philosophy, identidade, continuidade, consentimento, privacidade e direitos dos criadores.
- [ ] Se foi criada com IA, indiquei quais partes foram geradas/assistidas e como foram verificadas.
- [ ] Não declarei funcionalidades como concluídas sem testes/evidências.

Campos da PR:

- Resumo.
- Motivação/contexto.
- Componentes afetados.
- Tipo de mudança (breaking/non-breaking/docs/security).
- Como foi testada.
- Riscos e limitações conhecidos.
- Relacionamento com issue/SEP.

---

## 7. Segurança do repositório no GitHub

Abra `Settings → Security` / `Code security and analysis` e verifique os recursos disponíveis no plano e no tipo de repositório.

Ative, quando disponível e sem custo/licença adicional não autorizada:

1. Dependency graph.
2. Dependabot alerts.
3. Dependabot security updates.
4. Secret scanning.
5. Push protection.
6. Code scanning, se compatível com linguagens e plano.
7. Private vulnerability reporting, se disponível.

Não informe que um recurso está ativo apenas porque existe um arquivo de configuração. Consulte o estado real nas configurações do GitHub.

### 7.1 Dependabot

Crie `.github/dependabot.yml` somente para ecossistemas realmente presentes:

- `cargo` se existir `Cargo.toml`/`Cargo.lock`;
- `npm` se existir `package.json` e lockfile correspondente;
- `pip` se existir `pyproject.toml`, `requirements.txt` ou equivalente;
- `github-actions` se houver workflows.

Não adicione ecossistemas por previsão. Use intervalo semanal ou mensal razoável, limite de PRs e agrupamento de atualizações não críticas quando adequado. Não atualize dependências major automaticamente sem revisão.

### 7.2 SECURITY.md

Confirme que o documento explica:

- Versões em manutenção (não inventar versões; indicar política enquanto não houver releases).
- Como reportar vulnerabilidade privadamente.
- O que não deve ser enviado em issue pública.
- Prazo de confirmação/resposta como objetivo, não promessa irreal.
- Tratamento coordenado de divulgação.
- Como revogar/rotacionar credenciais comprometidas.

Não invente um endereço de e-mail de segurança. Se não existir canal privado configurado, documente isso como pendência e recomende habilitar Private Vulnerability Reporting ou adicionar contato confirmado pelo proprietário.

---

## 8. Branch principal e Ruleset

A branch padrão recomendada é `main`, mas confirme a branch atual antes de alterar qualquer configuração.

Prefira um **Repository Ruleset** direcionado à branch default em vez de criar regras duplicadas. Inspecione regras existentes primeiro; não crie sobreposição confusa.

### 8.1 Regras desejadas para `main`

Ative, quando suportado:

- Require a pull request before merging.
- Require conversation resolution before merging.
- Block force pushes.
- Block branch deletion.
- Require status checks somente depois que os checks reais existirem e tiverem passado pelo menos uma vez.
- Exigir linear history apenas se a estratégia de merge adotada for compatível.

### 8.2 Não bloquear o proprietário/único mantenedor

- Não exija uma aprovação de outra pessoa se não existir outro revisor com permissão de escrita.
- Não configure status checks fictícios ou com nomes que não existam.
- Não configure assinatura obrigatória de commits antes de verificar se o mantenedor consegue assinar commits e se a equipe concorda.
- Não configure merge queue para um projeto pequeno sem necessidade demonstrada.
- Não bloqueie a única conta administradora.
- Não conceda bypass a agentes de IA ou bots de automação.
- Se for necessário bypass emergencial, restrinja-o a administradores humanos e documente.

A regra principal é proteger `main` sem tornar o repositório impossível de manter.

### 8.3 Status checks

Antes de torná-los obrigatórios:

1. Crie workflow apropriado ao código real.
2. Execute-o em branch de trabalho.
3. Verifique o nome exato do check exibido pelo GitHub.
4. Confirme que ele passa em uma PR normal.
5. Só então adicione-o ao Ruleset.

---

## 9. GitHub Actions e CI

Inspecione os arquivos e linguagens reais antes de criar workflows.

### 9.1 Princípios obrigatórios

- `permissions` mínimo em cada workflow/job.
- Use `contents: read` como padrão quando for suficiente.
- Não forneça `contents: write` ou `pull-requests: write` sem necessidade explícita.
- Evite `pull_request_target` para executar código não confiável de PR.
- Não disponibilize secrets para código vindo de forks/PRs não confiáveis.
- Use `concurrency` para cancelar execuções redundantes quando seguro.
- Defina `timeout-minutes`.
- Faça checkout do commit correto e não execute scripts desconhecidos sem inspeção.
- Prefira actions oficiais ou auditadas e fixe actions externas em SHA completo imutável, não somente tags mutáveis.
- Não use `latest` para versões de ferramentas em builds reproduzíveis.
- Armazene artefatos somente quando necessário e com retenção razoável.
- Não faça deploy, release ou publicação automática nesta fase.

### 9.2 CI Rust

Só crie `.github/workflows/rust-ci.yml` se um workspace/código Rust real já existir.

Quando aplicável, incluir checks equivalentes a:

- `cargo fmt --all -- --check`
- `cargo check --workspace --all-targets`
- `cargo clippy --workspace --all-targets -- -D warnings` (ajustar se o projeto tiver lint allowances documentadas)
- `cargo test --workspace --all-targets`

Use `Cargo.lock` no repositório para aplicações/workspace executável. Não crie um `Cargo.toml` vazio apenas para fazer o CI parecer funcional. Se o repositório ainda for somente documentação, informe que o CI Rust ficará pendente.

### 9.3 CI de documentação e schemas

Se o repositório ainda for blueprint/documentação:

- Valide JSON schemas/fixtures existentes.
- Valide YAML de workflows e configurações.
- Verifique links internos quebrados, se houver ferramenta confiável já disponível.
- Faça Markdown lint somente se houver configuração e ferramenta estáveis.
- Não invente testes de código que não existe.

### 9.4 CI para Python/TypeScript

Somente adicione checks se houver código, manifests e lockfiles correspondentes. Respeite o gerenciador efetivamente usado (`uv`, `poetry`, `pip`, `pnpm`, `npm`, etc.). Não misture gerenciadores sem uma decisão documentada.

---

## 10. Labels, milestones e GitHub Projects

### 10.1 Milestones

Se o recurso estiver disponível e não houver organização diferente já adotada, crie milestones sem datas arbitrárias:

- `Phase 0 — Protocol Foundation`
- `Phase 1 — Seed Core MVP`
- `Phase 2 — P2P Foundation`
- `Phase 3 — Cardinal Ledger Research`
- `Phase 4 — Identity & Character Passport`
- `Phase 5 — Pandora & Yuri Foundation`
- `Phase 6 — World SDK Integration`
- `Phase 7 — Visitor & Migration Prototype`

Ajuste os nomes para corresponder ao roadmap oficial do repositório atual. Não prometa prazo nem data de entrega sem aprovação do proprietário.

### 10.2 Project board

Se for possível criar um GitHub Project, use colunas/status:

- Backlog
- Ready
- In Progress
- In Review
- Blocked
- Done

Campos recomendados:

- Component: Seed / Cardinal / Pandora / Yuri / SDK / Protocol / Docs.
- Priority: Critical / High / Medium / Low.
- Type: Bug / Feature / Security / Documentation / Research / Protocol.
- Phase: fase do roadmap oficial.
- AI-assisted: Yes / No.
- Security review required: Yes / No.
- Character integrity impact: Yes / No.

Não crie um Project duplicado se já existir um board adequado. Não atribua tarefas a pessoas que não aceitaram participar.

### 10.3 Discussions

Recomende ou habilite GitHub Discussions se o recurso estiver disponível e fizer sentido para a comunidade. Categorias sugeridas:

- Announcements
- Architecture & Research
- Ideas
- Q&A
- Show and Tell
- Seed Enhancement Proposals (SEP)

Não publique anúncios em nome do proprietário sem autorização.

---

## 11. Licença e propriedade intelectual

O projeto possui uma decisão de licença pendente ou documentada como proposta. Respeite `LICENSE_DECISION.md`.

- Não adicione `LICENSE` definitivo sem confirmação do proprietário.
- Apache-2.0 pode ser uma candidata para código/SDKs, mas não está adotada até haver confirmação e texto legal correto.
- CC BY 4.0 é uma candidata para documentação, não uma decisão automática.
- Não escolha licença para assets, exemplos de jogos, modelos 3D, músicas, imagens ou conteúdo de terceiros sem verificar direitos.
- Não adicione badges afirmando licença que ainda não foi adotada.
- A visibilidade pública do GitHub não equivale a uma licença open source.
- Não use nomes, marcas ou assets de franquias de terceiros como se fossem propriedade de THE SEED.

---

## 12. Política de IA, PANDORA e YURI no GitHub

O repositório deve estar preparado para desenvolvimento assistido por IA, mas a IA não deve receber privilégios maiores do que um colaborador humano confiável.

### 12.1 Permissões de PANDORA/Claude/agents

Permitir, quando autorizado:

- Ler código e documentação.
- Criar branches de trabalho.
- Criar commits em sua própria branch.
- Executar testes em ambiente isolado.
- Abrir Pull Requests.
- Anotar riscos e resultados.

Não permitir por padrão:

- Push direto para `main`.
- Aprovar ou fazer merge da própria PR.
- Alterar regrasets, secrets, colaboradores ou permissões.
- Acessar chaves de assinatura do CARDINAL.
- Acessar credenciais de produção do YURI/FreeLLM.
- Publicar releases ou pacotes.
- Implantar atualizações globais de PANDORA.

### 12.2 Registro de correções geradas por IA

Toda PR assistida por IA deve declarar:

- Agente/modelo usado, se conhecido e permitido divulgar.
- Escopo gerado ou modificado pela IA.
- Ferramentas executadas.
- Testes executados e resultados.
- Revisão humana realizada ou pendente.
- Riscos e limitações.

Não registre prompts privados, tokens, dados pessoais, saves ou conteúdo privado de personagens.

### 12.3 Global Fixes

Uma correção proposta por IA não é um Global Fix aprovado só por ter passado em testes locais.

O fluxo de referência é:

1. Incidente reproduzível.
2. Diagnóstico e causa provável.
3. Patch em sandbox/branch isolada.
4. Testes automatizados e de segurança.
5. Revisão independente.
6. Pull Request.
7. Aprovação conforme governança do componente.
8. Build reproduzível/proveniência verificável.
9. Release assinada por mecanismo separado do agente de IA.
10. Distribuição gradual e possibilidade de rollback.

Não configure automação que permita à IA aprovar e publicar seu próprio código.

---

## 13. Git workflow para esta tarefa

Se existe remote único e autenticado:

1. Preserve o estado atual do usuário.
2. Crie branch de trabalho com nome `chore/github-repository-setup`.
3. Faça as alterações locais necessárias.
4. Execute validações dos arquivos alterados.
5. Revise `git diff` e `git status`.
6. Não inclua arquivos de ambiente, build outputs, caches ou segredos.
7. Faça commit com mensagem clara, por exemplo `chore: prepare GitHub repository governance`.
8. Envie apenas a branch de trabalho, se as permissões permitirem.
9. Abra Pull Request para a branch padrão; não faça merge.
10. Se não for possível enviar a branch, mantenha as alterações locais e informe exatamente o que falta.

Se não existir remote ou houver ambiguidade:

- Não crie um repositório remoto por suposição.
- Prepare os arquivos locais que forem seguros.
- Informe que precisa do URL exato ou da seleção do repositório.

Nunca sobrescreva alterações não commitadas do usuário. Se houver alterações preexistentes, trabalhe em arquivos novos ou pare antes de editar os arquivos em conflito.

---

## 14. Verificações de conclusão

A tarefa só pode ser marcada como concluída depois de verificar cada item aplicável.

### 14.1 Repositório e apresentação

- [ ] Repositório correto identificado.
- [ ] Proprietário e URL remota confirmados.
- [ ] Branch padrão confirmada.
- [ ] Descrição e topics configurados ou pendências explicadas.
- [ ] README descreve com precisão o estágio real.
- [ ] Filosofia dos personagens preservada.

### 14.2 Arquivos de colaboração

- [ ] Issue templates criados/atualizados.
- [ ] Pull Request template criado/atualizado.
- [ ] CONTRIBUTING revisado.
- [ ] SECURITY revisado.
- [ ] AGENTS/AI_START_HERE consistentes.
- [ ] Labels configuradas sem duplicatas.
- [ ] Nenhuma licença foi inventada ou adotada sem autorização.

### 14.3 Segurança e automação

- [ ] Ruleset/branch protection verificado.
- [ ] `main` não permite force push nem exclusão acidental.
- [ ] Regra não bloqueia o único mantenedor.
- [ ] Required checks correspondem a checks reais.
- [ ] Actions usam permissões mínimas.
- [ ] Dependabot configurado somente para ecossistemas presentes.
- [ ] Recursos de segurança verificados remotamente.
- [ ] Nenhum secret foi exposto.

### 14.4 Entrega

- [ ] Diff revisado.
- [ ] Testes/validações executados.
- [ ] Branch enviada ou limitação explicada.
- [ ] PR aberta, se possível, sem merge automático.
- [ ] Estado remoto conferido novamente.
- [ ] Relatório final inclui links diretos e pendências.

---

## 15. Formato obrigatório do relatório final do Claude

Ao terminar, apresente uma tabela com os estados:

| Área | Estado | Evidência |
|---|---|---|
| Acesso ao repositório | Feito / Bloqueado | URL e método, sem credenciais |
| About e topics | Feito / Pendente | Valores verificados |
| Arquivos `.github` | Feito / Parcial / Pendente | Caminhos e diff |
| Labels | Feito / Parcial / Pendente | Quantidade e nomes |
| Ruleset `main` | Feito / Pendente | Regras realmente verificadas |
| Segurança | Feito / Parcial / Pendente | Recursos ativos e indisponíveis |
| CI | Feito / Parcial / Pendente | Workflows e execuções |
| Dependabot | Feito / Pendente | Ecossistemas configurados |
| Branch/PR | Feito / Pendente | Nome e URL da PR, se criada |
| Licença | Não alterada / Confirmar | Motivo |

Também informe:

1. O que foi alterado localmente.
2. O que foi alterado diretamente nas configurações do GitHub.
3. O que foi enviado ao remote.
4. Links para commits, workflows, rulesets e Pull Request, quando existirem.
5. Testes executados e resultados reais.
6. Recursos indisponíveis por plano, permissão ou falta de acesso.
7. Qualquer decisão que exige confirmação do proprietário.

**Não diga “tudo configurado” quando parte da configuração tiver sido apenas documentada ou recomendada.** Diferencie claramente `feito`, `preparado localmente`, `pendente de permissão` e `aguardando confirmação`.

---

## 16. Referências oficiais

Consulte a documentação oficial atual antes de fazer alterações de configuração, pois a interface e a disponibilidade dos recursos variam conforme o plano e o tipo de proprietário do repositório.

- Criar repositório: https://docs.github.com/en/repositories/creating-and-managing-repositories/creating-a-new-repository
- Rulesets: https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/about-rulesets
- Regras disponíveis em Rulesets: https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/available-rules-for-rulesets
- Criar Ruleset: https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/creating-rulesets-for-a-repository
- Proteção de branches: https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-protected-branches/about-protected-branches
- Segurança do repositório: https://docs.github.com/en/code-security/getting-started/quickstart-for-securing-your-repository
- Configurações de segurança: https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/enabling-features-for-your-repository/managing-security-and-analysis-settings-for-your-repository
- Uso seguro do GitHub Actions: https://docs.github.com/en/actions/reference/security/secure-use
- Dependabot: https://docs.github.com/en/code-security/dependabot/dependabot-alerts/about-dependabot-alerts
- Issue Forms: https://docs.github.com/en/issues/building-community/configuring-issue-templates-for-your-repository
- CODEOWNERS: https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-code-owners
- Licença: https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/licensing-a-repository

---

## 17. Primeira mensagem que Claude deve executar

Use esta ordem de trabalho:

> Leia este documento integralmente e depois faça uma auditoria somente de leitura do repositório THE SEED disponível no ambiente. Identifique o remote correto, branch padrão, permissões e estado atual. Leia também `README.md`, `THE_SEED_MASTER_SPEC.md`, `AGENTS.md`, `AI_START_HERE.md`, `GOVERNANCE.md`, `SECURITY.md`, `CONTRIBUTING.md` e `LICENSE_DECISION.md` se existirem. Em seguida, apresente um plano curto indicando o que pode executar sem risco, o que exige confirmação e o que depende de permissões GitHub. Não faça mudanças destrutivas nem altere visibilidade, licença, permissões, secrets ou regras críticas sem autorização. Depois execute as configurações seguras possíveis, valide tudo e entregue o relatório da seção 15. Se não tiver acesso ao GitHub remoto, não invente que fez alterações: prepare os arquivos locais e forneça instruções específicas para o que ficou pendente.

**Fim da especificação.**
