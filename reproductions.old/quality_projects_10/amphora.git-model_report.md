# Model report for file:///tmp/top-repos-quality-repos-mqwb7mko/amphora.git HEAD cdddd55c3b6ea5008ff28642f5aff754ff7cd304

### Dump

```json
{'created_at': '2021-08-24 16:39:26',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-5.4.0-81-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '21.0 kB',
 'tags': [],
 'uuid': 'bd5083db-a510-4259-aa42-46d93365a739',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-mqwb7mko/amphora.git cdddd55c3b6ea5008ff28642f5aff754ff7cd304

# javascript
47 rules, avg.len. 8.9
## train
PPCR: 0.956722
### report
macro
{'f1-score': 0.52898229282257,
 'precision': 0.5853452282476105,
 'recall': 0.5194254794274573,
 'support': 131158}
micro
{'f1-score': 0.9622211378642553,
 'precision': 0.9622211378642553,
 'recall': 0.9622211378642553,
 'support': 131158}
weighted
{'f1-score': 0.9557603141055945,
 'precision': 0.95317067438811,
 'recall': 0.9622211378642553,
 'support': 131158}
### report_full
macro
{'f1-score': 0.49718265769128095,
 'precision': 0.5853452282476105,
 'recall': 0.47899759583292867,
 'support': 137091}
micro
{'f1-score': 0.9409392020100728,
 'precision': 0.9622211378642553,
 'recall': 0.9205783020037785,
 'support': 137091}
weighted
{'f1-score': 0.9258630368192526,
 'precision': 0.9440385995221867,
 'recall': 0.9205783020037785,
 'support': 137091}
## test
PPCR: 0.924619
### report
macro
{'f1-score': 0.47045042663978426,
 'precision': 0.4787098407511698,
 'recall': 0.4880709713239131,
 'support': 19699}
micro
{'f1-score': 0.9281689425859181,
 'precision': 0.9281689425859181,
 'recall': 0.9281689425859181,
 'support': 19699}
weighted
{'f1-score': 0.9147338062436017,
 'precision': 0.9043479758021113,
 'recall': 0.9281689425859181,
 'support': 19699}
### report_full
macro
{'f1-score': 0.4191263907858351,
 'precision': 0.4787098407511698,
 'recall': 0.41743769976825945,
 'support': 21305}
micro
{'f1-score': 0.891815432640718,
 'precision': 0.9281689425859181,
 'recall': 0.858202299929594,
 'support': 21305}
weighted
{'f1-score': 0.8613540648419582,
 'precision': 0.8742436914001318,
 'recall': 0.858202299929594,
 'support': 21305}
```

## javascript
### Summary
32 rules, avg.len. 8.1

| | |
|-|-|
|Min support|133|
|Max support|22352|
|Min confidence|0.9210526347160339|
|Max confidence|0.9996374249458313|

### Configuration

```json
{'feature_extractor': {'cutoff_label_support': 80,
                       'debug_parsing': False,
                       'label_composites': '<cut>',
                       'left_features': ['length',
                                         'diff_offset',
                                         'diff_col',
                                         'diff_line',
                                         'internal_type',
                                         'label',
                                         'reserved',
                                         'roles'],
                       'left_siblings_window': 5,
                       'no_labels_on_right': True,
                       'node_features': ['start_line', 'start_col'],
                       'parent_features': ['internal_type', 'roles'],
                       'parents_depth': 2,
                       'return_sibling_indices': False,
                       'right_features': ['length', 'internal_type', 'reserved', 'roles'],
                       'right_siblings_window': 5,
                       'select_features_number': 500,
                       'selected_features': '<cut>'},
 'line_length_limit': 500,
 'lines_ratio_train_trigger': 0.2,
 'lower_bound_instances': 500,
 'optimizer': {'base_model_name_categories': ['sklearn.ensemble.RandomForestClassifier',
                                              'sklearn.tree.DecisionTreeClassifier'],
               'cv': 3,
               'max_depth_categories': [None, 5, 10],
               'max_features_categories': [None, 'auto'],
               'min_samples_leaf_max': 120,
               'min_samples_leaf_min': 90,
               'min_samples_split_max': 240,
               'min_samples_split_min': 180,
               'n_iter': 50,
               'n_jobs': -1},
 'overall_size_limit': 5242880,
 'random_state': 42,
 'test_dataset_ratio': 0.2,
 'trainable_rules': {'attribute_similarity_threshold': 0.98,
                     'base_model_name': 'sklearn.tree.DecisionTreeClassifier',
                     'confidence_threshold': 0.8,
                     'min_samples_leaf': 90,
                     'min_samples_split': 240,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.internal_type not in {StringLiteral}<br>	∧ ^1.roles in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.994. Support: 22352.` |
| 2 | `  -1.label in {<space>}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.957. Support: 750.` |
| 3 | `  -1.internal_type = StringLiteral<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles in {LITERAL}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = '<br>Confidence: 0.933. Support: 488.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles in {LITERAL}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.989. Support: 136.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.label not in {<space>}<br>	∧ -1.roles in {LITERAL}<br>	∧ +2.roles not in {EXPRESSION}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.967. Support: 404.` |
| 6 | `  -1.label not in {<space>}<br>	∧ -1.reserved = (<br>	∧ -1.roles not in {LITERAL}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.990. Support: 353.` |
| 7 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {LITERAL}<br>	∧ +1.reserved = )<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.939. Support: 205.` |
| 8 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {LITERAL}<br>	∧ +1.reserved = ]<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ∅<br>Confidence: 0.942. Support: 180.` |
| 9 | `  -1.diff_col ≤ 24<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(}<br>	∧ -1.roles not in {LITERAL}<br>	∧ +1.reserved not in {(, ), ]}<br>	∧ ^1.roles in {OPERATOR} and not in {IDENTIFIER}<br>⇒ y = ␣<br>Confidence: 0.965. Support: 11694.` |
| 10 | `  -1.roles in {EXPRESSION, STRING}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = '<br>Confidence: 0.949. Support: 3378.` |
| 11 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved = =<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.998. Support: 2113.` |
| 12 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {=}<br>	∧ +2.roles in {EXPRESSION}<br>	∧ ^1.internal_type = ConditionalExpression<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.993. Support: 219.` |
| 13 | `  -1.roles in {EXPRESSION} and not in {STRING}<br>	∧ +1.reserved not in {=, }}<br>	∧ ^1.internal_type not in {ConditionalExpression}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 16373.` |
| 14 | `  -1.reserved = (<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.roles not in {STRING}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.991. Support: 6003.` |
| 15 | `  -1.reserved not in {(}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ;<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ∅<br>Confidence: 1.000. Support: 4137.` |
| 16 | `  -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, }}<br>	∧ ^1.internal_type = ForStatement<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.995. Support: 319.` |
| 17 | `  •••start_col ≤ 10<br>	∧ -1.reserved = ;<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;, }}<br>	∧ +2.length ≥ 16<br>	∧ ^1.internal_type not in {ForStatement}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = ⏎⏎<br>Confidence: 0.921. Support: 133.` |
| 18 | `  •••start_line ≥ 164<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = '<br>Confidence: 0.985. Support: 493.` |
| 19 | `  •••start_line ≤ 128<br>	∧ -1.label in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;}<br>	∧ ^1.roles not in {IDENTIFIER, OPERATOR}<br>⇒ y = '<br>Confidence: 0.993. Support: 1218.` |
| 20 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {;}<br>	∧ +1.roles in {COMMENT}<br>	∧ ^1.roles in {FILE} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.996. Support: 1506.` |
| 21 | `  -1.internal_type not in {CommentLine}<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {}<br>	∧ -1.roles in {COMMENT} and not in {EXPRESSION}<br>	∧ -2.roles not in {BLOCK}<br>	∧ ^1.roles in {FILE} and not in {OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.999. Support: 348.` |
| 22 | `  -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.label not in {<-space>}<br>	∧ +1.reserved = }<br>	∧ ^1.roles not in {FILE, IDENTIFIER, OPERATOR}<br>⇒ y = ⏎␣⁻␣⁻<br>Confidence: 0.977. Support: 279.` |
| 23 | `  -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {}}<br>	∧ ^1.roles in {EXPRESSION} and not in {FILE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.995. Support: 490.` |
| 24 | `  -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {EXPRESSION, FILE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.982. Support: 256.` |
| 25 | `  -1.label not in {<space>}<br>	∧ -1.reserved = }<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), }}<br>	∧ ^1.internal_type = IfStatement<br>	∧ ^1.roles not in {EXPRESSION, FILE, IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.958. Support: 228.` |
| 26 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = )<br>	∧ ^1.roles not in {FILE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.964. Support: 598.` |
| 27 | `  •••start_col ≤ 68<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -5.diff_offset ≤ 6<br>	∧ +1.reserved not in {), ;}<br>	∧ ^1.internal_type = ObjectExpression<br>	∧ ^1.roles not in {FILE, IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.940. Support: 511.` |
| 28 | `  -1.label not in {<space>}<br>	∧ -1.reserved not in {(, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved = ,<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {FILE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.976. Support: 317.` |
| 29 | `  -1.label not in {<newline>, <space>}<br>	∧ -1.reserved = :<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ +1.reserved not in {), ,}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {FILE, IDENTIFIER, OPERATOR}<br>	∧ ^2.internal_type = SwitchStatement<br>⇒ y = ⏎␣⁺␣⁺<br>Confidence: 0.982. Support: 142.` |
| 30 | `  •••start_col ≥ 4<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved = (<br>	∧ -4.diff_offset ≥ 6<br>	∧ +1.reserved not in {), ,, ;}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {FILE, IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.932. Support: 771.` |
| 31 | `  •••start_col ≥ 4<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved = (<br>	∧ -4.diff_offset ≤ 5<br>	∧ +1.reserved not in {), ,}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {FILE, OPERATOR}<br>⇒ y = ∅<br>Confidence: 0.985. Support: 171.` |
| 32 | `  •••start_col ≥ 4<br>	∧ -1.label not in {<space>}<br>	∧ -1.reserved not in {(, ;, [, {, }}<br>	∧ -1.roles not in {EXPRESSION}<br>	∧ -2.reserved not in {(}<br>	∧ -3.reserved not in {,}<br>	∧ +1.reserved not in {), ,, ;}<br>	∧ +2.roles not in {ARGUMENT}<br>	∧ ^1.internal_type not in {ObjectExpression}<br>	∧ ^1.roles not in {FILE, IDENTIFIER, OPERATOR}<br>⇒ y = ␣<br>Confidence: 0.965. Support: 13952.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 8.09375, "max_conf": 0.9996374249458313, "max_support": 22352, "min_conf": 0.9210526347160339, "min_support": 133, "num_rules": 32}}
```
</details>
